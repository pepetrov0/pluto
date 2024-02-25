use regex::Regex;

const MIN_PASSWORD_LENGTH: usize = 12;

pub fn is_email(input: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map(|v| v.is_match(input))
        .unwrap_or_default()
}

pub fn is_password(input: &str) -> bool {
    input.len() >= MIN_PASSWORD_LENGTH
}
