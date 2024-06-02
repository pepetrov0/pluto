//! This module provides a implementation to change a user's email.

use secrecy::{ExposeSecret, Secret};
use tracing::instrument;

use super::{
    database::AnyTransaction,
    passwords,
    users::{User, UserError, UsersRepository},
    validations,
};

/// An error that might occur while changing a user's email.
#[derive(Debug, Clone, Copy)]
pub enum ChangeEmailError {
    EmailTaken,
    EmailInvalid,
    InvalidCredentials,
    Failure,
}

/// Validates the date required for a email change.
#[instrument(err, skip(tx))]
pub async fn validate_change_email(
    tx: &mut AnyTransaction,
    user: &User,
    new_email: &str,
) -> Result<(), ChangeEmailError> {
    // test if the new email is the same as the old one
    if user.email == new_email {
        return Ok(());
    }

    // test if email is taken
    if tx.find_user_by_email(new_email).await.ok().is_some() {
        return Err(ChangeEmailError::EmailTaken);
    }

    // test whether the email address is valid
    if !validations::email(new_email) {
        return Err(ChangeEmailError::EmailInvalid);
    }

    Ok(())
}

/// Changes a user's email.
#[instrument(err, skip(tx))]
pub async fn change_email(
    tx: &mut AnyTransaction,
    user: &User,
    new_email: &str,
    current_password: &Secret<String>,
) -> Result<(), ChangeEmailError> {
    // test if the new email is the same as the old one
    if user.email == new_email {
        return Ok(());
    }

    validate_change_email(tx, user, new_email).await?;
    let current_password = current_password.expose_secret().as_str();

    // validate password
    user.password
        .as_ref()
        .and_then(|v| passwords::verify_password(current_password, v.expose_secret().as_str()).ok())
        .ok_or(ChangeEmailError::InvalidCredentials)?;

    tx.update_user_email_by_id(user.id, new_email)
        .await
        .map(|_| ())
        .map_err(ChangeEmailError::from)
}

impl std::error::Error for ChangeEmailError {}
impl std::fmt::Display for ChangeEmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<UserError> for ChangeEmailError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::Message(_) | UserError::NotFound => Self::Failure,
        }
    }
}
