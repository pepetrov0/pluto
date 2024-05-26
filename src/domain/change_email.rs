//! This module provides a implementation to change a user's email.

use super::{
    database::AnyTransaction,
    users::{find_user_by_email, User},
    validations,
};

/// An error that might occur while changing a user's email.
pub enum ChangeEmailError {
    EmailTaken,
    EmailInvalid,
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
    _current_password: &str,
) -> Result<(), ChangeEmailError> {
    // test if the new email is the same as the old one
    if user.email == new_email {
        return Ok(());
    }

    validate_change_email(tx, user, new_email).await?;

    Err(ChangeEmailError::Failure)
}
