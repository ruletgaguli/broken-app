#[path = "../benches/support/before.rs"]
mod before;

use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even};
use std::hint::black_box;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let version = args.first().map(String::as_str).unwrap_or("after");
    assert!(matches!(version, "before" | "after"));
    let seconds = args
        .get(1)
        .map(|value| value.parse::<u64>().expect("нужны секунды"))
        .unwrap_or(10);
    let old = version == "before";
    let dedup: Vec<u64> = (0..5_000).rev().flat_map(|n| [n, n]).collect();
    let data: Vec<i64> = (0..50_000).collect();
    let bytes = vec![1; 1_000_000];
    let text = " Hello\tWORLD\n".repeat(4_096);
    let start = Instant::now();
    let mut cycles = 0_u64;
    while start.elapsed() < Duration::from_secs(seconds) {
        black_box(if old {
            before::slow_dedup(black_box(&dedup))
        } else {
            algo::slow_dedup(black_box(&dedup))
        });
        black_box(if old {
            before::slow_fib(black_box(32))
        } else {
            algo::slow_fib(black_box(32))
        });
        black_box(if old {
            before::sum_even(black_box(&data))
        } else {
            sum_even(black_box(&data))
        });
        black_box(if old {
            before::leak_buffer(black_box(&bytes))
        } else {
            leak_buffer(black_box(&bytes))
        });
        black_box(if old {
            before::normalize(black_box(&text))
        } else {
            normalize(black_box(&text))
        });
        black_box(if old {
            before::average_positive(black_box(&data))
        } else {
            average_positive(black_box(&data))
        });
        cycles += 1;
    }
    println!(
        "version={version}; cycles={cycles}; elapsed={:?}",
        start.elapsed()
    );
}
