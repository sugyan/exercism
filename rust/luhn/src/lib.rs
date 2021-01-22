/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|&c| c != ' ')
        .rev()
        .enumerate()
        .try_fold((0, 0), |(sum, len), (i, c)| {
            c.to_digit(10)
                .map(|u| if i % 2 == 0 { u } else { (u * 2) % 10 + u / 5 })
                .map(|u| (sum + u, len + 1))
        })
        .map_or(false, |(sum, len)| sum % 10 == 0 && len > 1)
}
