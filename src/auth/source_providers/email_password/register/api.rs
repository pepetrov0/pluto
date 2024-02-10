//! Implements the API relevant for user registration

use axum::{extract::State, response::Redirect, Form};
use chrono_tz::Tz;

use crate::{
    auth::{
        principal::NoAuthPrincipal, session::SessionWriteRepository,
        session_providers::cookie::SetCookieSession,
    },
    user::{UserReadonlyRepository, UserWriteRepository},
    validation, AppState,
};

use super::error::RegistrationError;

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub timezone: String,
}

pub async fn handler(
    _: NoAuthPrincipal,
    State(state): State<AppState>,
    Form(details): Form<RegisterForm>,
) -> Result<(SetCookieSession, Redirect), RegistrationError> {
    let mut tx = state
        .database
        .begin()
        .await
        .map_err(|_| RegistrationError::Unknown)?;

    // trim details (email)
    let details = RegisterForm {
        email: details.email.trim().to_owned(),
        ..details
    };

    // validate email
    if !validation::is_email(&details.email) {
        return Err(RegistrationError::InvalidEmail);
    }

    // validate password
    if !validation::is_password(&details.password) {
        return Err(RegistrationError::PasswordTooShort);
    }

    // validate matching passwords
    if details.password != details.confirm_password {
        return Err(RegistrationError::PasswordsMismatch);
    }

    // check if email is already taken
    if tx.find_user(&details.email).await.is_some() {
        return Err(RegistrationError::EmailTaken);
    }

    // attempt hashing the password
    let hash = match state.password_hasher.hash(details.password.as_bytes()) {
        Some(hash) => hash,
        None => return Err(RegistrationError::Unknown),
    };

    let timezone = Tz::from_str_insensitive(&details.timezone).unwrap_or_default();

    // attempt creating a user and a session
    let user = tx
        .create_user(details.email, Some(hash), timezone)
        .await
        .ok_or(RegistrationError::Unknown)?;

    // create a session
    let session = tx
        .create_session(user.id)
        .await
        .ok_or(RegistrationError::Unknown)?;

    tx.commit().await.map_err(|_| RegistrationError::Unknown)?;
    Ok((SetCookieSession(session), Redirect::to("/")))
}
