use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut hs = HashSet::new();
    helper(limit, factors, 1, &mut hs);
    hs.iter().sum()
}

fn helper(limit: u32, factors: &[u32], start: u32, hs: &mut HashSet<u32>) {
    if let Some(&factor) = factors.first() {
        if factor == 0 {
            return;
        }
        helper(limit, &factors[1..], start, hs);
        for n in (1..=(limit - 1) / factor).map(|i| start * i * factor) {
            if n < limit && !hs.contains(&n) {
                hs.insert(n);
                helper(limit, &factors[1..], n, hs);
            }
        }
    }
}
