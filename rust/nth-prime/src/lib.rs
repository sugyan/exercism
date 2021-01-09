pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::with_capacity(n as usize + 1);
    primes.push(2u64);
    while primes.len() <= n as usize {
        if let Some(prime) = (primes[primes.len() - 1] + 1..).find(|&m| {
            primes
                .iter()
                .take_while(|&e| e * e <= m)
                .all(|&e| m % e != 0)
        }) {
            primes.push(prime);
        }
    }
    primes[n as usize] as u32
}
