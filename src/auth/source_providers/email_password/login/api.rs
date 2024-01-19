//! Implements the API relevant for user registration

use axum::{extract::State, response::Redirect, Form};

use crate::{
    auth::{principal::NoAuthPrincipal, session_providers::cookie::SetCookieSession},
    validation, AppState,
};

use super::error::LoginError;

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

pub async fn handler(
    _: NoAuthPrincipal,
    State(state): State<AppState>,
    Form(details): Form<RegisterForm>,
) -> Result<(SetCookieSession, Redirect), LoginError> {
    if !validation::is_email(&details.email) {
        return Err(LoginError::InvalidCredentials);
    }

    let user = state
        .user_repository
        .find_user_with_password(&details.email)
        .await
        .ok_or(LoginError::InvalidCredentials)?;

    let hash = match user.password {
        Some(password) => password,
        None => return Err(LoginError::InvalidCredentials),
    };

    if !state
        .password_hasher
        .verify(details.password.as_bytes(), &hash)
    {
        return Err(LoginError::InvalidCredentials);
    }

    match state.session_repository.create_session(user.id).await {
        Some(session) => Ok((SetCookieSession(session), Redirect::to("/"))),
        None => Err(LoginError::Unknown),
    }
}
