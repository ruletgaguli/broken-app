#[path = "support/before.rs"]
mod before;

use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

fn comparisons(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(200));
    group.measurement_time(Duration::from_secs(1));
    for size in [100, 10_000] {
        let data: Vec<u64> = (0..size / 2).rev().flat_map(|n| [n, n]).collect();
        assert_eq!(before::slow_dedup(&data), algo::slow_dedup(&data));
        group.bench_with_input(BenchmarkId::new("dedup_before", size), &data, |b, v| {
            b.iter(|| before::slow_dedup(black_box(v)))
        });
        group.bench_with_input(BenchmarkId::new("dedup_after", size), &data, |b, v| {
            b.iter(|| algo::slow_dedup(black_box(v)))
        });
    }
    for n in [20, 32] {
        assert_eq!(before::slow_fib(n), algo::slow_fib(n));
        group.bench_with_input(BenchmarkId::new("fib_before", n), &n, |b, n| {
            b.iter(|| before::slow_fib(black_box(*n)))
        });
        group.bench_with_input(BenchmarkId::new("fib_after", n), &n, |b, n| {
            b.iter(|| algo::slow_fib(black_box(*n)))
        });
    }
    let bytes: Vec<u8> = (0..1_000_000).map(|n| (n % 256) as u8).collect();
    assert_eq!(before::leak_buffer(&bytes), leak_buffer(&bytes));
    group.bench_function("count_before/1000000", |b| {
        b.iter(|| before::leak_buffer(black_box(&bytes)))
    });
    group.bench_function("count_after/1000000", |b| {
        b.iter(|| leak_buffer(black_box(&bytes)))
    });
    let text = " Hello\tWORLD\n".repeat(4_096);
    assert_eq!(before::normalize(&text), normalize(&text));
    group.bench_function("normalize_before/53248", |b| {
        b.iter(|| before::normalize(black_box(&text)))
    });
    group.bench_function("normalize_after/53248", |b| {
        b.iter(|| normalize(black_box(&text)))
    });
    let data: Vec<i64> = (0..50_000).collect();
    assert_eq!(before::average_positive(&data), average_positive(&data));
    group.bench_function("average_before/50000", |b| {
        b.iter(|| before::average_positive(black_box(&data)))
    });
    group.bench_function("average_after/50000", |b| {
        b.iter(|| average_positive(black_box(&data)))
    });
    group.bench_function("sum_before/50000", |b| {
        b.iter(|| before::sum_even(black_box(&data)))
    });
    group.bench_function("sum_after/50000", |b| b.iter(|| sum_even(black_box(&data))));
    group.finish();
}

criterion_group!(benches, comparisons);
criterion_main!(benches);
