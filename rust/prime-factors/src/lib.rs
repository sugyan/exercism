pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut primes = vec![2; 1];
    let mut ret = Vec::new();
    loop {
        let last = primes[primes.len() - 1];
        if last * last > n {
            break;
        }
        while n % last == 0 {
            ret.push(last);
            n /= last;
        }
        if let Some(prime) = (last + 1..).take_while(|&i| i * i <= n).find(|&i| {
            primes
                .iter()
                .take_while(|&p| p * p <= i)
                .all(|&p| i % p != 0)
        }) {
            primes.push(prime);
        }
    }
    if n > 1 {
        ret.push(n);
    }
    ret
}
