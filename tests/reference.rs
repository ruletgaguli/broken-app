use reference_app as reference;

use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even};

#[test]
fn matches_reference_on_deterministic_inputs() {
    for len in 0..64 {
        let values: Vec<i64> = (0..len)
            .map(|index| (index * 17 % 31) as i64 - 15)
            .collect();
        let bytes: Vec<u8> = values
            .iter()
            .map(|value| value.unsigned_abs() as u8)
            .collect();
        let unique: Vec<u64> = bytes.iter().map(|value| u64::from(*value)).collect();
        assert_eq!(sum_even(&values), reference::sum_even(&values));
        assert_eq!(leak_buffer(&bytes), reference::leak_buffer(&bytes));
        assert_eq!(
            average_positive(&values),
            reference::average_positive(&values)
        );
        assert_eq!(
            algo::slow_dedup(&unique),
            reference::algo::fast_dedup(&unique)
        );
    }
    for n in 0..=93 {
        assert_eq!(algo::slow_fib(n), reference::algo::fast_fib(n));
    }
    for input in [
        "",
        "\tHello\nWORLD\r",
        "ПрИвЕт\u{a0}МиР",
        "ΟΣ",
        "İΣ\u{2003}Θ",
    ] {
        assert_eq!(normalize(input), reference::normalize(input));
    }
    let total = reference::concurrency::race_increment(8, 2);
    assert_eq!(total, broken_app::concurrency::race_increment(8, 2));
    assert_eq!(reference::concurrency::read_after_sleep(), total);
}
