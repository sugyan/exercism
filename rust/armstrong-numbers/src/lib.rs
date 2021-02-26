use std::cmp::Ordering;

pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut nums = Vec::new();
    while n > 0 {
        nums.push(n % 10);
        n /= 10;
    }
    if nums.iter().sum::<u32>() == 1 {
        return num == 1;
    }
    for i in 1.. {
        match nums.iter().map(|&n| n.pow(i)).sum::<u32>().cmp(&num) {
            Ordering::Less => {}
            Ordering::Equal => return true,
            Ordering::Greater => return false,
        }
    }
    unreachable!()
}
