#![forbid(unsafe_code)]

pub mod algo;
pub mod concurrency;

/// Сумма чётных значений. Переполнение явно отклоняется во всех профилях.
pub fn sum_even(values: &[i64]) -> i64 {
    values
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .fold(0_i64, |sum, value| {
            sum.checked_add(value).expect("сумма не помещается в i64")
        })
}

/// Подсчёт ненулевых байтов без копирования и выделения памяти.
/// Историческое имя сохранено для совместимости.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|byte| **byte != 0).count()
}

/// Удаляет Unicode-пробелы и приводит к нижнему регистру.
/// Для ASCII переиспользует буфер; Unicode сохраняет контекстные правила регистра.
pub fn normalize(input: &str) -> String {
    let mut result: String = input.split_whitespace().collect();
    if result.is_ascii() {
        result.make_ascii_lowercase();
        result
    } else {
        result.to_lowercase()
    }
}

/// Среднее только положительных значений; при их отсутствии возвращает ноль.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .copied()
        .filter(|value| *value > 0)
        .fold((0.0, 0_usize), |(sum, count), value| {
            (sum + value as f64, count + 1)
        });
    if count == 0 { 0.0 } else { sum / count as f64 }
}

/// Безопасная замена прежнего примера use-after-free: оба чтения при живом владельце.
pub fn use_after_free() -> i32 {
    let value = 42;
    value + value
}
