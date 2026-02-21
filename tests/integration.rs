use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    // Ожидается (5 + 15) / 2 = 10, но текущая реализация делит на все элементы.
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn test_sum_even_regression() {
    assert_eq!(sum_even(&[1, 2, 3, 4, 5, 6]), 12);
    assert_eq!(sum_even(&[]), 0);
    assert_eq!(sum_even(&[1, 3, 5]), 0);
}

#[test]
fn test_average_positive_regression() {
    assert_eq!(average_positive(&[10, -5, 20]), 15.0);
    assert_eq!(average_positive(&[-1, -2, -3]), 0.0);
    assert_eq!(average_positive(&[]), 0.0);
}

#[test]
fn test_fib_regression() {
    assert_eq!(algo::slow_fib(0), 0);
    assert_eq!(algo::slow_fib(1), 1);
    assert_eq!(algo::slow_fib(10), 55);
    assert_eq!(algo::slow_fib(40), 102_334_155);
}

#[test]
fn test_dedup_regression() {
    let input = vec![1, 2, 2, 3, 1, 4, 4, 4];
    let mut result = algo::slow_dedup(&input);
    result.sort_unstable();
    assert_eq!(result, vec![1, 2, 3, 4]);
    let empty_input: Vec<u64> = vec![];
    assert!(algo::slow_dedup(&empty_input).is_empty());
}
