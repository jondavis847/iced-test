use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use uuid::Uuid;

fn bench_usize_hashmap(c: &mut Criterion) {
    c.bench_function("usize hashmap insert", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
        })
    });

    c.bench_function("usize hashmap lookup", |b| {
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(i, i);
        }
        b.iter(|| {
            for i in 0..1000 {
                map.get(&i);
            }
        })
    });
}

fn bench_uuid_hashmap(c: &mut Criterion) {
    c.bench_function("uuid hashmap insert", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for _ in 0..1000 {
                let key = Uuid::new_v4();
                map.insert(key, key);
            }
        })
    });

    c.bench_function("uuid hashmap lookup", |b| {
        let mut map = HashMap::new();
        let mut keys = Vec::new();
        for _ in 0..1000 {
            let key = Uuid::new_v4();
            keys.push(key);
            map.insert(key, key);
        }
        b.iter(|| {
            for key in &keys {
                map.get(key);
            }
        })
    });
}

criterion_group!(benches, bench_usize_hashmap, bench_uuid_hashmap);
criterion_main!(benches);
