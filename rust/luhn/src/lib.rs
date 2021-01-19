/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut v = Vec::new();
    for &b in code.as_bytes() {
        match b {
            b'0'..=b'9' => v.push(b - b'0'),
            b' ' => {}
            _ => return false,
        }
    }
    if v.len() <= 1 {
        return false;
    }
    v.iter()
        .rev()
        .enumerate()
        .map(|(i, &u)| if i % 2 == 0 { u } else { (u * 2) % 10 + u / 5 })
        .sum::<u8>()
        % 10
        == 0
}
