/// Возвращает отсортированные уникальные значения за O(n log n).
/// Историческое имя сохранено; выделяется один буфер вместо сортировки каждой вставки.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = values.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}

/// Число Фибоначчи за O(n) времени и O(1) памяти.
/// Значения n > 93 не помещаются в u64 и явно отклоняются.
pub fn slow_fib(n: u64) -> u64 {
    assert!(n <= 93, "число Фибоначчи не помещается в u64");
    if n == 0 {
        return 0;
    }
    let (mut previous, mut current) = (0_u64, 1_u64);
    for _ in 2..=n {
        (previous, current) = (current, previous + current);
    }
    current
}
