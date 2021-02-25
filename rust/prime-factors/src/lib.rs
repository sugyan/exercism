pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut ret = Vec::new();
    while n > 1 {
        if let Some(factor) = (2..=n).find(|m| n % m == 0) {
            ret.push(factor);
            n /= factor;
        }
    }
    ret
}
