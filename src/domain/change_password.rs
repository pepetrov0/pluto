//! This module provides a implementation to change a user's password.

use secrecy::{ExposeSecret, Secret};
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
#[instrument(err)]
pub fn validate_change_password(
    user: &User,
    new_password: &Secret<String>,
    confirm_new_password: &Secret<String>,
) -> Result<(), ChangePasswordError> {
    let new_password = new_password.expose_secret().as_str();
    let confirm_new_password = confirm_new_password.expose_secret().as_str();

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
#[instrument(err, skip(tx))]
pub async fn change_password(
    tx: &mut AnyTransaction,
    user: &User,
    new_password: &Secret<String>,
    confirm_new_password: &Secret<String>,
    current_password: &Secret<String>,
) -> Result<(), ChangePasswordError> {
    validate_change_password(user, new_password, confirm_new_password)?;

    let new_password = new_password.expose_secret().as_str();
    let current_password = current_password.expose_secret().as_str();

    // validate password
    user.password
        .as_ref()
        .and_then(|v| passwords::verify_password(current_password, v.expose_secret().as_str()).ok())
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
