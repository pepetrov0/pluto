//! This module provides a implementation to change a user's email.

use super::{
    database::AnyTransaction,
    passwords,
    users::{
        find_user_by_email, find_user_with_password_by_id, update_user_email_by_id, User, UserError,
    },
    validations,
};

/// An error that might occur while changing a user's email.
pub enum ChangeEmailError {
    EmailTaken,
    EmailInvalid,
    InvalidCredentials,
    Failure,
}

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
    if find_user_by_email(tx, new_email).await.is_ok() {
        return Err(ChangeEmailError::EmailTaken);
    }

    // test whether the email address is valid
    if !validations::email(new_email) {
        return Err(ChangeEmailError::EmailInvalid);
    }

    Ok(())
}

pub async fn change_email(
    tx: &mut AnyTransaction,
    user: &User,
    new_email: &str,
    current_password: &str,
) -> Result<(), ChangeEmailError> {
    // test if the new email is the same as the old one
    if user.email == new_email {
        return Ok(());
    }

    validate_change_email(tx, user, new_email).await?;

    let user = find_user_with_password_by_id(tx, user.id)
        .await
        .map_err(ChangeEmailError::from)?;

    // validate password
    user.password
        .as_ref()
        .and_then(|v| passwords::verify_password(current_password, v.as_str()).ok())
        .ok_or(ChangeEmailError::InvalidCredentials)?;

    update_user_email_by_id(tx, user.id, new_email)
        .await
        .map(|_| ())
        .map_err(ChangeEmailError::from)
}

impl From<UserError> for ChangeEmailError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::Database(_) | UserError::UserNotFound => Self::Failure,
        }
    }
}
