//! Implements various data validation methods

use regex::Regex;

const MIN_PASSWORD_LENGTH: usize = 12;

/// Validate a string being an email
pub fn is_email(input: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map(|v| v.is_match(input))
        .unwrap_or_default()
}

/// Validate a string being a valid candidate for password
pub fn is_password(input: &str) -> bool {
    input.len() >= MIN_PASSWORD_LENGTH
}
