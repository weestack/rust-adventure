use std::hint::black_box;
use criterion::{criterion_group, Criterion};


pub fn heap<T: Clone>(a: T) -> T {
    a.clone()
}

pub fn stack<T: Copy>(a: T) -> T {
    a
}

fn bench_heap(c: &mut Criterion) {
    c.bench_function("heap", |b| {
        b.iter(|| heap(black_box((1, 2, (), "hello".to_string()))))
    });
}

fn bench_copy(c: &mut Criterion) {
    c.bench_function("copy", |b| {
        b.iter(|| stack(black_box((1, 2, (), "hello"))))
    });
}

criterion_group!(benches, bench_heap, bench_copy);
