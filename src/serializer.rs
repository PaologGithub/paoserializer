use alloc::vec;
use alloc::vec::Vec;

use serde::Serialize;

use crate::errors::SerializeError;

pub fn serialize<T: Serialize>(object: T) -> Result<Vec<u8>, SerializeError> {
    let bytes: Vec<u8> = postcard::to_allocvec(&object).map_err(SerializeError::PostCardError)?;

    let payload: Vec<u8> = if bytes.len() > 150 {
        let mut compressed = vec![0u8; zstd_safe::compress_bound(bytes.len())];

        let written = zstd_safe::compress(&mut compressed[..], &bytes, 3)
            .map_err(SerializeError::ZStdError)?;
        compressed.truncate(written);

        compressed
    } else {
        bytes.clone()
    };

    let hash: u32 = crc32fast::hash(&payload);
    let hash_bytes: [u8; 4] = hash.to_le_bytes();

    let mut data: Vec<u8> = Vec::with_capacity(8 + payload.len());
    data.extend_from_slice(&hash_bytes);
    data.extend_from_slice(b"PAO");
    data.push(if bytes.len() > 150 { 1 } else { 0 });
    data.extend_from_slice(&payload);

    Ok(data)
}
