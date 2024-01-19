//! Implements various data validation methods

use regex::Regex;

/// Validate a string being an email
pub fn is_email(input: &str) -> bool {
    Regex::new(r"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$")
        .map(|v| v.is_match(input))
        .unwrap_or_default()
}
