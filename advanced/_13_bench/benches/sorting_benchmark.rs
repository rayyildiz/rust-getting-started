use _13_bench::{sort_algo, sort_algo2};
use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    c.bench_function("sort", |b| {
        b.iter(|| sort_algo());
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
