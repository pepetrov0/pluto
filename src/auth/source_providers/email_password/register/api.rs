//! Implements the API relevant for user registration

use axum::{extract::State, response::Redirect, Form};

use crate::{
    auth::{principal::NoAuthPrincipal, session_providers::cookie::SetCookieSession},
    validation, AppState,
};

use super::error::RegistrationError;

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub async fn handler(
    _: NoAuthPrincipal,
    State(state): State<AppState>,
    Form(details): Form<RegisterForm>,
) -> Result<(SetCookieSession, Redirect), RegistrationError> {
    if validation::is_email(&details.email) {
        return Err(RegistrationError::InvalidEmail);
    }

    if details.password != details.confirm_password {
        return Err(RegistrationError::PasswordsMismatch);
    }

    if state
        .user_repository
        .find_user(&details.email)
        .await
        .is_some()
    {
        return Err(RegistrationError::EmailTaken);
    }

    let hash = match state.password_hasher.hash(details.password) {
        Some(hash) => hash,
        None => return Err(RegistrationError::Unknown),
    };

    match state
        .user_repository
        .create_user(details.email, Some(hash))
        .await
    {
        // create a session and redirect to /
        Some(user) => match state.session_repository.create_session(user.id).await {
            Some(session) => Ok((SetCookieSession(session), Redirect::to("/"))),
            None => Err(RegistrationError::SessionCreationError),
        },
        None => Err(RegistrationError::Unknown),
    }
}
