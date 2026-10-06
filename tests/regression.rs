use broken_app::{algo, average_positive, concurrency, leak_buffer, normalize, sum_even};

#[test]
fn empty_and_last_even_element() {
    assert_eq!(sum_even(&[]), 0);
    assert_eq!(sum_even(&[2]), 2);
    assert_eq!(sum_even(&[-4, 3, 8]), 4);
}

#[test]
fn buffer_is_released() {
    for _ in 0..16 {
        assert_eq!(leak_buffer(&[0, 1, 255, 0]), 2);
    }
    assert_eq!(leak_buffer(&[]), 0);
}

#[test]
fn owned_value_remains_alive() {
    assert_eq!(broken_app::use_after_free(), 84);
}

#[test]
fn whitespace_and_unicode() {
    assert_eq!(normalize(" \tHello\nWORLD\r\u{a0} "), "helloworld");
    assert_eq!(normalize("\u{2003}ΟΣ\t"), "ος");
    assert_eq!(normalize("\t\n\u{a0}"), "");
}

#[test]
fn positive_mean_edge_cases() {
    assert_eq!(average_positive(&[]), 0.0);
    assert_eq!(average_positive(&[-1, 0, -10]), 0.0);
    assert_eq!(average_positive(&[-100, 2, 0, 6]), 4.0);
    assert_eq!(average_positive(&[i64::MAX, i64::MAX]), i64::MAX as f64);
}

#[test]
fn dedup_edge_cases() {
    assert_eq!(algo::slow_dedup(&[]), []);
    assert_eq!(algo::slow_dedup(&[7, 7, 7]), [7]);
    assert_eq!(algo::slow_dedup(&[u64::MAX, 0, u64::MAX]), [0, u64::MAX]);
}

#[test]
fn fib_edges() {
    assert_eq!(algo::slow_fib(0), 0);
    assert_eq!(algo::slow_fib(1), 1);
    assert_eq!(algo::slow_fib(2), 1);
    assert_eq!(algo::slow_fib(10), 55);
    assert_eq!(algo::slow_fib(93), 12_200_160_415_121_876_738);
}

#[test]
#[should_panic(expected = "число Фибоначчи не помещается в u64")]
fn fib_overflow_is_explicit() {
    algo::slow_fib(94);
}

#[test]
#[should_panic(expected = "сумма не помещается в i64")]
fn sum_overflow_is_explicit() {
    sum_even(&[i64::MAX - 1, 2]);
}

#[test]
fn counter_is_synchronized_and_calls_are_independent() {
    assert_eq!(concurrency::race_increment(16, 4), 64);
    assert_eq!(concurrency::read_after_sleep(), 64);
    let handles: Vec<_> = (1..=4)
        .map(|threads| std::thread::spawn(move || concurrency::race_increment(8, threads)))
        .collect();
    for (index, handle) in handles.into_iter().enumerate() {
        assert_eq!(handle.join().unwrap(), 8 * (index as u64 + 1));
    }
    concurrency::reset_counter();
    assert_eq!(concurrency::read_after_sleep(), 0);
    assert_eq!(concurrency::race_increment(0, 2), 0);
    assert_eq!(concurrency::race_increment(8, 0), 0);
}
