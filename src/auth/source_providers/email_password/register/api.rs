//! Implements the API relevant for user registration

use axum::{extract::State, response::Redirect, Form};
use chrono_tz::Tz;

use crate::{
    auth::{principal::NoAuthPrincipal, session::SessionRepository, session_providers::cookie::SetCookieSession}, user::UserRepository, validation, AppState
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
    State(mut state): State<AppState>,
    Form(details): Form<RegisterForm>,
) -> Result<(SetCookieSession, Redirect), RegistrationError> {
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
    if state
        .database
        .find_user(&details.email)
        .await
        .is_some()
    {
        return Err(RegistrationError::EmailTaken);
    }

    // attempt hashing the password
    let hash = match state.password_hasher.hash(details.password.as_bytes()) {
        Some(hash) => hash,
        None => return Err(RegistrationError::Unknown),
    };

    let timezone = Tz::from_str_insensitive(&details.timezone).unwrap_or_default();

    // attempt creating a user and a session
    match state
        .database
        .create_user(details.email, Some(hash), timezone)
        .await
    {
        // create a session and redirect to /
        Some(user) => match state.database.create_session(user.id).await {
            Some(session) => Ok((SetCookieSession(session), Redirect::to("/"))),
            None => Err(RegistrationError::SessionCreationError),
        },
        None => Err(RegistrationError::Unknown),
    }
}
