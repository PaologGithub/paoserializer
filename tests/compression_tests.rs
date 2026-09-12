use serde::{Serialize, Deserialize};

use paoserializer::{deserializer::deserialize, serializer::serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Profile {
    pub name: String,
    pub uuid: String,
    pub password: String,
    pub age: u8
}

const BIG_STRING: &str = r#"According to all known laws of aviation, there is no way a bee should be able to fly.
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

#[test]
pub fn test_compression() {
    let profile = Profile {
        name: BIG_STRING.to_string(),
        uuid: BIG_STRING.to_string(),
        password: BIG_STRING.to_string(),
        age: 3
    };

    let serialized = serialize(&profile).unwrap();
    assert_eq!(serialized[7], 1);

    let object: Profile = deserialize(serialized.as_slice()).unwrap();
    assert_eq!(object, profile);
}