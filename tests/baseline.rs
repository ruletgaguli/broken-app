#[path = "../benches/support/before.rs"]
mod before;

#[test]
fn safe_baseline_matches_fixed_behavior() {
    let values = [-4, 0, 2, 7, 8];
    assert_eq!(before::sum_even(&values), broken_app::sum_even(&values));
    assert_eq!(
        before::average_positive(&values),
        broken_app::average_positive(&values)
    );
    assert_eq!(
        before::leak_buffer(&[0, 1, 0, 255]),
        broken_app::leak_buffer(&[0, 1, 0, 255])
    );
    for text in [" Hello\tWORLD\n", "ΟΣ\u{a0}ПрИвЕт", ""] {
        assert_eq!(before::normalize(text), broken_app::normalize(text));
    }
    assert_eq!(
        before::slow_dedup(&[9, 1, 9, 0, 1]),
        broken_app::algo::slow_dedup(&[9, 1, 9, 0, 1])
    );
    for n in 0..=10 {
        assert_eq!(before::slow_fib(n), broken_app::algo::slow_fib(n));
    }
}
