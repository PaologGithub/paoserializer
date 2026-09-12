use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use serde::{Deserialize, Serialize};

use paoserializer::deserializer::deserialize;
use paoserializer::serializer::serialize;

#[derive(Serialize, Deserialize, Clone)]
struct Profile {
    pub name: String,
    pub uuid: String,
    pub password: String,
    pub age: u8,
}

fn small_profile() -> Profile {
    Profile {
        name: "Paolog".to_string(),
        uuid: "0000-0000-0000-0000".to_string(),
        password: "12345678".to_string(),
        age: 3,
    }
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

fn large_profile() -> Profile {
    Profile {
        name: BIG_STRING.to_string(),
        uuid: BIG_STRING.to_string(),
        password: BIG_STRING.to_string(),
        age: 3,
    }
}

fn bench_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize");

    group.bench_function(BenchmarkId::new("uncompressed", "small_profile"), |b| {
        let profile = small_profile();
        b.iter(|| serialize(std::hint::black_box(&profile)).unwrap())
    });

    group.bench_function(BenchmarkId::new("compressed", "large_profile"), |b| {
        let profile = large_profile();
        b.iter(|| serialize(std::hint::black_box(&profile)).unwrap())
    });

    group.finish();
}

fn bench_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize");

    let small_bytes = serialize(&small_profile()).unwrap();
    let large_bytes = serialize(&large_profile()).unwrap();

    group.bench_function(BenchmarkId::new("uncompressed", "small_profile"), |b| {
        b.iter(|| deserialize::<Profile>(std::hint::black_box(small_bytes.clone().as_slice())).unwrap())
    });

    group.bench_function(BenchmarkId::new("compressed", "large_profile"), |b| {
        b.iter(|| deserialize::<Profile>(std::hint::black_box(large_bytes.clone().as_slice())).unwrap())
    });

    group.finish();
}

fn bench_basic_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_serialization");

    group.bench_function("small_profile", |b| {
        let profile = small_profile();
        b.iter(|| {
            let bytes = serialize(std::hint::black_box(&profile)).unwrap();
            deserialize::<Profile>(bytes.as_slice()).unwrap()
        })
    });

    group.bench_function("large_profile", |b| {
        let profile = large_profile();
        b.iter(|| {
            let bytes = serialize(std::hint::black_box(&profile)).unwrap();
            deserialize::<Profile>(bytes.as_slice()).unwrap()
        })
    });

    group.finish();
}

criterion_group!(benches, bench_serialize, bench_deserialize, bench_basic_serialization);
criterion_main!(benches);