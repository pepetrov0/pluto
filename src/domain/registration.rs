//! This module facilitates user registration.

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use tracing::instrument;

use super::{
    database::AnyTransaction,
    sessions::{create_session, Session, SessionError},
    users::{create_user, find_user_by_email, User, UserError},
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

/// Registers a new user.
#[instrument(err, skip_all)]
pub async fn register(
    transaction: &mut AnyTransaction,
    email: &str,
    password: &str,
    confirm_password: &str,
) -> Result<User, RegistrationError> {
    // test if email is taken
    if find_user_by_email(transaction, email).await.is_ok() {
        return Err(RegistrationError::EmailTaken);
    }

    // test whether the email address is valid
    if !validations::email(email) {
        return Err(RegistrationError::EmailInvalid);
    }

    // test if passwords match
    if password != confirm_password {
        return Err(RegistrationError::PasswordsMismatch);
    }

    // assess password
    if !validations::password_strength(password, &[email]) {
        return Err(RegistrationError::WeakPassword);
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let password = argon
        .hash_password(password.as_bytes(), &salt)
        .map_err(RegistrationError::from)?
        .to_string();

    create_user(transaction, email, Some(&password))
        .await
        .map_err(RegistrationError::from)
}

/// Registers a new user and immediately authenticate them.
#[instrument(err, skip_all)]
pub async fn register_and_authenticate(
    transaction: &mut AnyTransaction,
    email: &str,
    password: &str,
    confirm_password: &str,
    agent: &str,
) -> Result<(User, Session), RegistrationError> {
    let user = register(transaction, email, password, confirm_password).await?;
    let session = create_session(transaction, user.id, agent)
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

impl From<argon2::password_hash::Error> for RegistrationError {
    fn from(_: argon2::password_hash::Error) -> Self {
        Self::Failure
    }
}

impl From<UserError> for RegistrationError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::Database(_) | UserError::UserNotFound => Self::Failure,
        }
    }
}

impl From<SessionError> for RegistrationError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::Database(_) | SessionError::SessionNotFound => Self::Failure,
        }
    }
}
