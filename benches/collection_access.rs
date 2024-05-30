use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn bench_array(c: &mut Criterion) {
    let array: [i32; 1000] = [0; 1000];
    c.bench_function("array access", |b| {
        b.iter(|| {
            let _ = black_box(array[500]);
        })
    });
}

fn bench_vec(c: &mut Criterion) {
    let vec: Vec<i32> = vec![0; 1000];
    c.bench_function("vec access", |b| {
        b.iter(|| {
            let _ = black_box(vec[500]);
        })
    });
}

fn bench_hashmap(c: &mut Criterion) {
    let mut hashmap: HashMap<usize, i32> = HashMap::new();
    for i in 0..1000 {
        hashmap.insert(i, i as i32);
    }
    c.bench_function("hashmap access", |b| {
        b.iter(|| {
            let _ = black_box(hashmap.get(&500));
        })
    });
}

criterion_group!(benches, bench_array, bench_vec, bench_hashmap);
criterion_main!(benches);
