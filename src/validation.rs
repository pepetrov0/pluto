use regex::Regex;

pub fn email(email: &str) -> bool {
    Regex::new(r"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$")
        .map(|v| v.is_match(email))
        .unwrap_or_default()
}
