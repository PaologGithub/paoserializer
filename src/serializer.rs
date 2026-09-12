use std::error::Error;

use serde::Serialize;

pub fn serialize<T: Serialize>(object: T) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = postcard::to_stdvec(&object)?;

    let compressed;
    let payload: &[u8];

    if bytes.len() > 150 {
        compressed = zstd::encode_all(bytes.as_slice(), 0)?;
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