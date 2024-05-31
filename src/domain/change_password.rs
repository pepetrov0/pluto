//! This module provides a implementation to change a user's password.

use tracing::instrument;

use super::{users::User, validations};

#[derive(Debug, Clone, Copy)]
pub enum ChangePasswordError {
    PasswordsMismatch,
    WeakPassword,
    Failure,
}

/// Validates the date required for a password change.
#[instrument(err, skip(new_password, confirm_new_password))]
pub fn validate_change_password(
    user: &User,
    new_password: &str,
    confirm_new_password: &str,
) -> Result<(), ChangePasswordError> {
    // assess password
    if !validations::password_strength(new_password, &[user.email.as_str()]) {
        return Err(ChangePasswordError::WeakPassword);
    }

    // test if passwords match
    if new_password != confirm_new_password {
        return Err(ChangePasswordError::PasswordsMismatch);
    }

    Ok(())
}

impl std::error::Error for ChangePasswordError {}
impl std::fmt::Display for ChangePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
