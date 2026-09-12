use serde::Serialize;

use crate::errors::SerializeError;

pub fn serialize<T: Serialize>(object: T) -> Result<Vec<u8>, SerializeError> {
    let bytes = postcard::to_stdvec(&object)
        .map_err(|e| SerializeError::PostCardError(e))?;

    let mut compressed;
    let payload: &[u8];

    if bytes.len() > 150 {
        compressed = vec![0u8; zstd_safe::compress_bound(bytes.len())];
        let written = zstd_safe::compress(&mut compressed, bytes.as_slice(), 3)
            .map_err(|e| SerializeError::ZStdError(e))?;
        compressed.truncate(written);
        payload = compressed.as_slice();
    } else {
        payload = &bytes;
    }

    let hash: u32 = crc32fast::hash(payload);
    let hash_bytes: [u8; 4] = hash.to_le_bytes();

    let mut data: Vec<u8> = Vec::new();
    
    data.extend_from_slice(&hash_bytes);
    data.extend_from_slice(b"PAO");
    data.extend_from_slice(&[if bytes.len() > 150 {1} else {0}]);
    data.extend_from_slice(payload);

    Ok(data)
}