use aska::service::fetchservice::parse;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multimap::MultiMap;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bebra", |b| b.iter(|| parse(black_box(MultiMap::new()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
