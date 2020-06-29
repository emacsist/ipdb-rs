use criterion::{criterion_group, criterion_main, Criterion};

use ipdbv4_rust::find;

fn criterion_benchmark(c: &mut Criterion) {
    find("58.250.137.36", "CN");
    c.bench_function("ipdbv4", |b| b.iter(|| find("58.250.137.36", "CN")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
