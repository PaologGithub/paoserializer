use alloc::vec;
use serde::Deserialize;
use zstd_safe::get_frame_content_size;

use crate::errors::DeserializeError;

pub fn deserialize<T: for<'a> Deserialize<'a>>(bytes: &[u8]) -> Result<T, DeserializeError> {
    if bytes.len() < 8 {
        return Err(DeserializeError::InvalidLength);
    };

    let hash: &[u8] = &bytes[..4];
    let magic: &[u8] = &bytes[4..7];
    let compress: bool = bytes[7] == 1;
    let data: &[u8] = &bytes[8..];

    if magic != b"PAO" {
        return Err(DeserializeError::InvalidMagic);
    };

    let data_hash: u32 = crc32fast::hash(data);
    let hash_bytes: [u8; 4] = data_hash.to_le_bytes();

    if hash_bytes != hash {
        return Err(DeserializeError::InvalidHash);
    };

    if compress {
        let content_size = match get_frame_content_size(data) {
            Ok(Some(size)) => size as usize,
            Ok(None) => {
                return Err(DeserializeError::InvalidLength);
            }
            Err(_) => {
                return Err(DeserializeError::InvalidLength);
            }
        };

        let mut decompressed = vec![0u8; content_size];
        let written = zstd_safe::decompress(&mut decompressed[..], data)
            .map_err(|e| DeserializeError::ZStdError(e))?;
        decompressed.truncate(written);

        let object: T = postcard::from_bytes(&decompressed)
            .map_err(|e| DeserializeError::PostCardError(e))?;
        return Ok(object);
    }
    let object: T = postcard::from_bytes(data)
        .map_err(|e| DeserializeError::PostCardError(e))?;
    Ok(object)
}