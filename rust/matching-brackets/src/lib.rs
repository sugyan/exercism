macro_rules! match_pop {
    ($s:ident, $c:literal) => {
        if $s.pop() != Some($c) {
            return false;
        }
    };
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' => match_pop!(stack, '['),
            '}' => match_pop!(stack, '{'),
            ')' => match_pop!(stack, '('),
            _ => {}
        }
    }
    stack.is_empty()
}
