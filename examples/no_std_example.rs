#![no_std]
extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use paoserializer::{deserializer::deserialize, serializer::serialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct NoStdStruct {
    pub list: Vec<String>,
    pub test: bool,
}

// Run with `cargo run --example no_std_test --no-default-features`
pub fn main() {
    let hello = "Hello There".to_string();
    let mut list = Vec::new();
    for _ in 0..10000 {
        list.push(hello.clone());
    }

    let test = NoStdStruct { list, test: true };

    let serialized = serialize(&test).unwrap();
    let object: NoStdStruct = deserialize(serialized.as_slice()).unwrap();

    assert!(object.test);
    assert_eq!(object.list, test.list);
}
