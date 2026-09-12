use serde::{Deserialize, Serialize};

use paoserializer::{deserializer::deserialize, serializer::serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Profile {
    pub name: String,
    pub uuid: String,
    pub password: String,
    pub age: u8,
}

#[test]
pub fn test_serialize() {
    let profile = Profile {
        name: "Paolog".to_string(),
        uuid: "0000-0000-0000-0000".to_string(),
        password: "12345678".to_string(),
        age: 3,
    };

    let first = serialize(&profile).unwrap();
    let second = serialize(profile).unwrap();
    assert_eq!(first, second);

    assert_eq!(
        first,
        [
            91, 14, 235, 28, 80, 65, 79, 0, 6, 80, 97, 111, 108, 111, 103, 19, 48, 48, 48, 48, 45,
            48, 48, 48, 48, 45, 48, 48, 48, 48, 45, 48, 48, 48, 48, 8, 49, 50, 51, 52, 53, 54, 55,
            56, 3
        ]
    );
}

#[test]
pub fn test_deserialize() {
    let profile = Profile {
        name: "Paolog".to_string(),
        uuid: "0000-0000-0000-0000".to_string(),
        password: "12345678".to_string(),
        age: 3,
    };
    let bytes: [u8; 45] = [
        91, 14, 235, 28, 80, 65, 79, 0, 6, 80, 97, 111, 108, 111, 103, 19, 48, 48, 48, 48, 45, 48,
        48, 48, 48, 45, 48, 48, 48, 48, 45, 48, 48, 48, 48, 8, 49, 50, 51, 52, 53, 54, 55, 56, 3,
    ];

    let object: Profile = deserialize(&bytes).unwrap();

    assert_eq!(profile, object);
}
