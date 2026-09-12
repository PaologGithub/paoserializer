use std::{error::Error, fmt::Display, io::{self, ErrorKind}};

use serde::Deserialize;
use zstd_safe::get_frame_content_size;

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength,
    InvalidMagic,
    InvalidHash
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::InvalidLength => write!(f, "Invalid bytes length"),
            DeserializeError::InvalidMagic => write!(f, "Invalid magic"),
            DeserializeError::InvalidHash => write!(f, "Invalid hash")
        }
    }
}

impl Error for DeserializeError {}

pub fn deserialize<T: for<'a> Deserialize<'a>>(bytes: Vec<u8>) -> Result<T, Box<dyn Error>> {
    if bytes.len() < 8 {
        return Err(Box::new(DeserializeError::InvalidLength));
    };

    let hash: &[u8] = &bytes[..4];
    let magic: &[u8] = &bytes[4..7];
    let compress: bool = bytes[7] == 1;
    let data: &[u8] = &bytes[8..];

    if magic != b"PAO" {
        return Err(Box::new(DeserializeError::InvalidMagic));
    };

    let data_hash: u32 = crc32fast::hash(data);
    let hash_bytes: [u8; 4] = data_hash.to_le_bytes();

    if hash_bytes != hash {
        return Err(Box::new(DeserializeError::InvalidHash));
    };

    if compress {
        let content_size = match get_frame_content_size(data) {
            Ok(Some(size)) => size as usize,
            Ok(None) => {
                return Err(Box::new(DeserializeError::InvalidLength));
            }
            Err(_) => {
                return Err(Box::new(DeserializeError::InvalidLength));
            }
        };

        let mut decompressed = vec![0u8; content_size];
        let written = zstd_safe::decompress(&mut decompressed, data)
            .map_err(|_| io::Error::from(ErrorKind::InvalidData))?;
        decompressed.truncate(written);

        let object: T = postcard::from_bytes(&decompressed)?;
        return Ok(object);
    }
    let object: T = postcard::from_bytes(data)?;
    Ok(object)
}