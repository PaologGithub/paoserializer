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

#[cfg(test)]
mod data_serializer_tests {
    use serde::{Serialize, Deserialize};
    use crate::serializer::serialize;

    #[derive(Serialize, Deserialize)]
    struct Profile {
        pub name: String,
        pub uuid: String,
        pub password: String,
        pub age: u8
    }

    #[test]
    pub fn test_serialize() {
        let profile = Profile {
            name: "Paolog".to_string(),
            uuid: "0000-0000-0000-0000".to_string(),
            password: "12345678".to_string(),
            age: 3
        };

        let first = serialize(&profile).unwrap();
        let second = serialize(profile).unwrap();
        assert_eq!(first, second);
        
        assert_eq!(first, [91, 14, 235, 28, 80, 65, 79, 0, 6, 80, 97, 111, 108, 111, 103, 19, 48, 48, 48, 48, 45, 48, 48, 48, 48, 45, 48, 48, 48, 48, 45, 48, 48, 48, 48, 8, 49, 50, 51, 52, 53, 54, 55, 56, 3]);
    }

    #[test]
    pub fn test_serialize_compression() {
        let big_string = r#"According to all known laws of aviation, there is no way a bee should be able to fly.
Its wings are too small to get its fat little body off the ground.
The bee, of course, flies anyway because bees don't care what humans think is impossible.
Yellow, black. Yellow, black. Yellow, black. Yellow, black.
Ooh, black and yellow!
Let's shake it up a little.
Barry! Breakfast is ready!
Coming!
Hang on a second.
Hello?
Barry?
Adam?
Can you believe this is happening?
I can't.
I'll pick you up.
Looking sharp.
Use the stairs, Your father paid good money for those.
Sorry. I'm excited.
Here's the graduate.
We're very proud of you, son."#;

        let profile = Profile {
            name: big_string.to_string(),
            uuid: big_string.to_string(),
            password: big_string.to_string(),
            age: 3
        };

        let serialized = serialize(profile).unwrap();
        assert_eq!(serialized, [138, 78, 227, 34, 80, 65, 79, 1, 40, 181, 47, 253, 0, 88, 197, 12, 0, 166, 97, 92, 42, 160, 37, 73, 7, 255, 255, 255, 255, 255, 0, 237, 79, 218, 150, 54, 32, 114, 147, 164, 200, 77, 136, 220, 132, 249, 125, 243, 63, 189, 40, 28, 203, 178, 235, 110, 7, 61, 82, 181, 55, 62, 49, 76, 0, 80, 0, 83, 0, 198, 115, 117, 235, 122, 35, 114, 110, 125, 103, 242, 237, 175, 133, 11, 127, 183, 159, 13, 207, 9, 211, 230, 220, 136, 131, 222, 66, 190, 253, 92, 6, 62, 141, 167, 229, 86, 241, 51, 141, 227, 31, 54, 95, 118, 214, 103, 75, 213, 55, 176, 213, 70, 77, 224, 145, 61, 55, 184, 206, 154, 101, 202, 215, 194, 111, 63, 97, 154, 218, 30, 71, 65, 128, 4, 1, 157, 9, 167, 240, 158, 24, 94, 173, 128, 115, 164, 28, 184, 140, 27, 154, 93, 43, 15, 28, 148, 203, 80, 230, 101, 22, 61, 235, 21, 166, 170, 207, 12, 23, 167, 222, 168, 154, 103, 195, 227, 93, 176, 77, 103, 118, 120, 164, 241, 135, 205, 99, 120, 78, 93, 200, 13, 201, 205, 237, 113, 100, 111, 200, 52, 158, 138, 192, 65, 137, 185, 109, 201, 83, 245, 17, 100, 255, 210, 6, 204, 215, 42, 16, 7, 117, 193, 240, 56, 226, 28, 18, 166, 44, 25, 6, 213, 89, 207, 170, 250, 153, 72, 93, 105, 220, 237, 59, 195, 131, 115, 88, 48, 21, 156, 195, 97, 215, 74, 132, 131, 18, 219, 35, 249, 120, 182, 79, 24, 34, 156, 66, 194, 116, 217, 131, 83, 120, 9, 179, 157, 245, 205, 136, 40, 152, 205, 224, 133, 215, 26, 26, 28, 132, 103, 220, 91, 159, 10, 42, 62, 23, 1, 0, 16, 91, 190, 193, 237, 59, 178, 79, 110, 27, 243, 215, 50, 241, 108, 69, 26, 56, 40, 27, 93, 194, 150, 112, 145, 45, 91, 15, 4, 249, 165, 64, 28, 189, 214, 118, 192, 65, 201, 171, 250, 109, 145, 191, 100, 187, 60, 237, 38, 79, 138, 204, 91, 170, 70, 228, 205, 237, 129, 27, 184, 41, 54, 39, 23, 56, 136, 105, 168, 248, 9, 83, 101, 55, 225, 32, 198, 41, 135, 6, 11, 0, 95, 71, 143, 252, 23, 133, 16, 240, 178, 51, 228, 86, 19, 93, 149, 216, 253, 117, 236, 96, 203, 235, 132, 6, 185, 114, 115, 39, 211, 64, 33, 18, 148, 20]);
    }
}