// Диагностические тесты запускаются только в распакованном исходнике с UB.
#[test]
fn owned_value_remains_alive() {
    assert_eq!(unsafe { broken_app::use_after_free() }, 84);
}

#[test]
fn buffer_is_released() {
    for _ in 0..16 {
        assert_eq!(broken_app::leak_buffer(&[0, 1, 255, 0]), 2);
    }
}

#[test]
fn counter_is_synchronized_and_calls_are_independent() {
    assert_eq!(broken_app::concurrency::race_increment(16, 4), 64);
}
