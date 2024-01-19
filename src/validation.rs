//! Implements various data validation methods

use regex::Regex;

/// Validate a string being an email
pub fn is_email(input: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map(|v| v.is_match(input))
        .unwrap_or_default()
}
