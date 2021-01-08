pub fn raindrops(n: u32) -> String {
    let mut ret = String::new();
    if n % 3 == 0 {
        ret += "Pling";
    }
    if n % 5 == 0 {
        ret += "Plang";
    }
    if n % 7 == 0 {
        ret += "Plong";
    }
    if ret.is_empty() {
        ret += &n.to_string();
    }
    ret
}
