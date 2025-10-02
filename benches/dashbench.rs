use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dashbench::{
    dashmap_prealloc, dashmap_simple, dashmap_threaded, hashmap_prealloc, hashmap_simple,
    hashmap_threaded, hashmap_threaded_rwlock, hashmap_range_query_simple
};

const ITER: usize = 9000;

fn hashmap_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap");
    group.bench_function("hashmap_simple", |b| b.iter(|| hashmap_simple(black_box(ITER))));

    group.bench_function("hashmap_prealloc", |b| b.iter(|| hashmap_prealloc(black_box(ITER))));
    group.finish();
}

fn dashmap_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("DashMap");
    group.bench_function("dashmap_simple", |b| b.iter(|| dashmap_simple(ITER)));

    group.bench_function("dashmap_prealloc", |b| b.iter(|| dashmap_prealloc(ITER)));
    group.finish();
}

fn rayon_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rayon");
    group.bench_function("hashmap_threaded_arc_mutex", |b| {
        b.iter(|| hashmap_threaded(ITER))
    });
    group.bench_function("hashmap_threaded_rwlock", |b| {
        b.iter(|| hashmap_threaded_rwlock(ITER))
    });
    group.bench_function("dashmap_threaded", |b| b.iter(|| dashmap_threaded(ITER)));
    group.finish();
}

fn hashmap_bench_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMapRangeQuery");
    group.bench_function("hashmap_range_query_simple", |b| b.iter(|| hashmap_range_query_simple(ITER)));

    // group.bench_function("hashmap_prealloc", |b| b.iter(|| hashmap_prealloc(ITER)));
    group.finish();
}

criterion_group!(hashmap, hashmap_bench, hashmap_bench_range);
criterion_group!(dashmap, dashmap_bench);
criterion_group!(rayon, rayon_bench);
criterion_main!(hashmap, dashmap, rayon);
