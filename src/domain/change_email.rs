//! This module provides a implementation to change a user's email.

use super::{database::AnyTransaction, users::find_user_by_email, validations};

/// An error that might occur while changing a user's email.
pub enum ChangeEmailError {
    EmailTaken,
    EmailInvalid,
    Failure,
}

pub async fn validate_change_email(
    tx: &mut AnyTransaction,
    new_email: &str,
) -> Result<(), ChangeEmailError> {
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
