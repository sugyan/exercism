pub fn is_armstrong_number(num: u32) -> bool {
    let nums = num
        .to_string()
        .as_bytes()
        .iter()
        .map(|&b| (b - b'0') as u32)
        .collect::<Vec<_>>();
    nums.iter().map(|&n| n.pow(nums.len() as u32)).sum::<u32>() == num
}
