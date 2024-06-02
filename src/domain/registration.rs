//! This module facilitates user registration.

use secrecy::{ExposeSecret, Secret};
use tracing::instrument;

use super::{
    database::AnyTransaction,
    passwords::{self, PasswordError},
    sessions::{Session, SessionError, SessionsRepository},
    users::{User, UserError, UsersRepository},
    validations,
};

#[derive(Debug, Clone, Copy)]
pub enum RegistrationError {
    EmailTaken,
    EmailInvalid,
    PasswordsMismatch,
    WeakPassword,
    Failure,
}

/// Validates register data.
#[instrument(err, skip(tx))]
pub async fn validate_register(
    tx: &mut AnyTransaction,
    email: &str,
    password: &Secret<String>,
    confirm_password: &Secret<String>,
) -> Result<(), RegistrationError> {
    let password = password.expose_secret().as_str();
    let confirm_password = confirm_password.expose_secret().as_str();

    // test whether the email address is valid
    if !validations::email(email) {
        return Err(RegistrationError::EmailInvalid);
    }

    // test if email is taken
    if tx.find_user_by_email(email).await.is_ok() {
        return Err(RegistrationError::EmailTaken);
    }

    // assess password
    if !validations::password_strength(password, &[email]) {
        return Err(RegistrationError::WeakPassword);
    }

    // test if passwords match
    if password != confirm_password {
        return Err(RegistrationError::PasswordsMismatch);
    }

    Ok(())
}

/// Registers a new user.
#[instrument(err, skip(tx))]
pub async fn register(
    tx: &mut AnyTransaction,
    email: &str,
    password: &Secret<String>,
    confirm_password: &Secret<String>,
) -> Result<User, RegistrationError> {
    validate_register(tx, email, password, confirm_password).await?;

    let password = password.expose_secret().as_str();
    let password = passwords::hash_password(password).map_err(RegistrationError::from)?;
    tx.create_user(email, Some(Secret::from(password)))
        .await
        .map_err(RegistrationError::from)
}

/// Registers a new user and immediately authenticate them.
#[instrument(err, skip(tx))]
pub async fn register_and_authenticate(
    tx: &mut AnyTransaction,
    email: &str,
    password: &Secret<String>,
    confirm_password: &Secret<String>,
    agent: &str,
) -> Result<(User, Session), RegistrationError> {
    let user = register(tx, email, password, confirm_password).await?;
    let session = tx
        .create_session(user.id, agent)
        .await
        .map_err(RegistrationError::from)?;
    Ok((user, session))
}

impl std::error::Error for RegistrationError {}
impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<PasswordError> for RegistrationError {
    fn from(_: PasswordError) -> Self {
        Self::Failure
    }
}

impl From<UserError> for RegistrationError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::Message(_) | UserError::NotFound => Self::Failure,
        }
    }
}

impl From<SessionError> for RegistrationError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::Message(_) | SessionError::NotFound => Self::Failure,
        }
    }
}
