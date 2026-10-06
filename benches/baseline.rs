#[path = "support/before.rs"]
mod before;

use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even};
use std::hint::black_box;
use std::time::Instant;

fn measure(label: &str, mut operation: impl FnMut()) {
    operation();
    let mut samples = Vec::with_capacity(9);
    for _ in 0..9 {
        let start = Instant::now();
        for _ in 0..10 {
            operation();
        }
        samples.push(start.elapsed().as_nanos() / 10);
    }
    samples.sort_unstable();
    println!("{label},{}", samples[4]);
}

fn main() {
    let version = std::env::var("BENCH_VERSION").unwrap_or_else(|_| "after".into());
    assert!(matches!(version.as_str(), "before" | "after"));
    let old = version == "before";
    let data: Vec<i64> = (0..50_000).collect();
    let dedup: Vec<u64> = (0..5_000).rev().flat_map(|n| [n, n]).collect();
    let bytes: Vec<u8> = (0..1_000_000).map(|n| (n % 256) as u8).collect();
    let text = " Hello\tWORLD\n".repeat(4_096);
    println!("version={version}; median of 9 samples, 10 calls/sample; ns/call");
    measure("sum_even/50000", || {
        black_box(if old {
            before::sum_even(black_box(&data))
        } else {
            sum_even(black_box(&data))
        });
    });
    measure("fib/32", || {
        black_box(if old {
            before::slow_fib(black_box(32))
        } else {
            algo::slow_fib(black_box(32))
        });
    });
    measure("dedup/10000", || {
        black_box(if old {
            before::slow_dedup(black_box(&dedup))
        } else {
            algo::slow_dedup(black_box(&dedup))
        });
    });
    measure("count/1000000", || {
        black_box(if old {
            before::leak_buffer(black_box(&bytes))
        } else {
            leak_buffer(black_box(&bytes))
        });
    });
    measure("normalize/53248", || {
        black_box(if old {
            before::normalize(black_box(&text))
        } else {
            normalize(black_box(&text))
        });
    });
    measure("average/50000", || {
        black_box(if old {
            before::average_positive(black_box(&data))
        } else {
            average_positive(black_box(&data))
        });
    });
}
