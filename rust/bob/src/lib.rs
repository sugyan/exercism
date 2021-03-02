pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = message.trim().ends_with('?');
    let is_yell = message.chars().any(char::is_alphabetic) && message.to_uppercase() == message;
    match (is_question, is_yell) {
        (false, false) => "Whatever.",
        (false, true) => "Whoa, chill out!",
        (true, false) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
    }
}
