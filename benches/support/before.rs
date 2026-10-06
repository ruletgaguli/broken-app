// Безопасная точка «до»: исправлена корректность, сохранены дорогие алгоритмы.
// slow_dedup и slow_fib совпадают с исходником в artifacts/original.tar.gz.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            out.push(*v);
            out.sort_unstable();
        }
    }
    out
}

pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|value| value % 2 == 0).sum()
}

pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    boxed.iter().filter(|byte| **byte != 0).count()
}

pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

pub fn average_positive(values: &[i64]) -> f64 {
    let positives: Vec<_> = values.iter().copied().filter(|value| *value > 0).collect();
    if positives.is_empty() {
        return 0.0;
    }
    positives.iter().map(|value| *value as f64).sum::<f64>() / positives.len() as f64
}
