use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Каждый вызов имеет свой счётчик; после join публикуется готовый результат.
/// Параллельные вызовы не сбрасывают и не изменяют состояние друг друга.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    let total = iterations
        .checked_mul(threads)
        .and_then(|value| u64::try_from(value).ok())
        .expect("счётчик не помещается в u64");
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for handle in handles {
        handle.join().expect("поток счётчика завершился с паникой");
    }
    let result = counter.load(Ordering::SeqCst);
    debug_assert_eq!(result, total);
    COUNTER.store(result, Ordering::SeqCst);
    result
}

/// Читает последнее опубликованное значение без задержки.
/// При одновременных вызовах это снимок последнего завершившегося вызова.
pub fn read_after_sleep() -> u64 {
    COUNTER.load(Ordering::SeqCst)
}

/// Сбрасывает опубликованный снимок; уже работающие вызовы могут опубликовать новый.
pub fn reset_counter() {
    COUNTER.store(0, Ordering::SeqCst);
}
