use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Clone, Copy)]
struct MyStructCopy {
    a: u32,
    b: u64,
    c: f64,
}

#[derive(Clone)]
struct MyStructClone {
    a: u32,
    b: u64,
    c: Vec<f64>,
}

#[derive(Clone)]
struct MyStructCloneString {
    a: String,
    b: u64,
    c: Vec<f64>,
}

fn benchmark_copy(c: &mut Criterion) {
    let s = MyStructCopy { a: 1, b: 2, c: 3.0 };
    c.bench_function("copy", |b| {
        b.iter(|| {
            let s2 = black_box(s);
            let _ = s2;
        })
    });
}

fn benchmark_clone(c: &mut Criterion) {
    let s = MyStructClone { a: 1, b: 2, c: vec![3.0,4.0] };
    c.bench_function("clone", |b| {
        b.iter(|| {
            let s2 = black_box(s.clone());
            let _ = s2;
        })
    });
}

fn benchmark_clone_string(c: &mut Criterion) {
    let s = MyStructCloneString { a: "1".to_string(), b: 2, c: vec![3.0,4.0] };
    c.bench_function("clone", |b| {
        b.iter(|| {
            let s2 = black_box(s.clone());
            let _ = s2;
        })
    });
}

criterion_group!(benches, benchmark_copy, benchmark_clone, benchmark_clone_string);
criterion_main!(benches);