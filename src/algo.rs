/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    if values.is_empty() { return vec![]; }
    let mut out = values.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
pub fn slow_fib(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
