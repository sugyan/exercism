pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with('?') && is_yell(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with('?') => "Sure.",
        m if is_yell(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_yell(message: &str) -> bool {
    message.chars().any(char::is_alphabetic) && message.to_ascii_uppercase() == message
}
