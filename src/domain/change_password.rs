//! This module provides a implementation to change a user's password.

use tracing::instrument;

use super::{
    database::AnyTransaction,
    passwords::{self, PasswordError},
    users::{User, UserError, UsersRepository},
    validations,
};

#[derive(Debug, Clone, Copy)]
pub enum ChangePasswordError {
    PasswordsMismatch,
    WeakPassword,
    InvalidCredentials,
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

/// Changes a user's password.
#[instrument(err, skip(tx, new_password, confirm_new_password))]
pub async fn change_password(
    tx: &mut AnyTransaction,
    user: &User,
    new_password: &str,
    confirm_new_password: &str,
    current_password: &str,
) -> Result<(), ChangePasswordError> {
    validate_change_password(user, new_password, confirm_new_password)?;

    // validate password
    user.password
        .as_ref()
        .and_then(|v| passwords::verify_password(current_password, v.as_str()).ok())
        .ok_or(ChangePasswordError::InvalidCredentials)?;

    // hash password
    let password = passwords::hash_password(new_password).map_err(ChangePasswordError::from)?;

    tx.update_user_password_by_id(user.id, Some(password.as_str()))
        .await
        .map(|_| ())
        .map_err(ChangePasswordError::from)
}

impl std::error::Error for ChangePasswordError {}
impl std::fmt::Display for ChangePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<UserError> for ChangePasswordError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::Message(_) | UserError::NotFound => Self::Failure,
        }
    }
}

impl From<PasswordError> for ChangePasswordError {
    fn from(_: PasswordError) -> Self {
        Self::Failure
    }
}
