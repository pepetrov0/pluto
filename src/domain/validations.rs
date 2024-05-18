//! This module facilitates data validations.

use email_address::EmailAddress;

/// Validates an email address.
pub fn email(value: &str) -> bool {
    EmailAddress::is_valid(value)
}

/// Validates a password's strength.
pub fn password_strength(value: &str, other_inputs: &[&str]) -> bool {
    zxcvbn::zxcvbn(value, other_inputs)
        .map(|v| v.score() >= 3)
        .unwrap_or_default()
}
