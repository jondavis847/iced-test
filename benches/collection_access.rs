use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use indexmap::IndexMap;
use rustc_hash::FxHashMap;

const N: usize = 1000;

fn bench_array(c: &mut Criterion) {
    let array: [i32; 1000] = [0; N];
    c.bench_function("array access", |b| {
        b.iter(|| {
            let _ = black_box(array[500]);
        })
    });
}

fn bench_vec(c: &mut Criterion) {
    let vec: Vec<i32> = vec![0; N];
    c.bench_function("vec access", |b| {
        b.iter(|| {
            let _ = black_box(vec[500]);
        })
    });
}

fn bench_hashmap(c: &mut Criterion) {
    let mut hashmap: HashMap<usize, i32> = HashMap::new();
    for i in 0..N {
        hashmap.insert(i, i as i32);
    }
    c.bench_function("hashmap access", |b| {
        b.iter(|| {
            let _ = black_box(hashmap.get(&500));
        })
    });
}

fn bench_fxhashmap(c: &mut Criterion) {
    let mut map = FxHashMap::default();
    for i in 0..N {
        map.insert(i, i as i32);
    }

    c.bench_function("FxHashMap lookup", |b| {
        b.iter(|| {
            for i in 0..N {
                black_box(map.get(&i));
            }
        })
    });
}

fn bench_indexmap(c: &mut Criterion) {
    let mut map = IndexMap::new();
    for i in 0..N {
        map.insert(i, i as i32);
    }

    c.bench_function("IndexMap lookup", |b| {
        b.iter(|| {
            for i in 0..N {
                black_box(map.get(&i));
            }
        })
    });
}


criterion_group!(benches, bench_array, bench_vec, bench_hashmap, bench_fxhashmap, bench_indexmap);
criterion_main!(benches);
