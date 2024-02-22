//! Implements the API relevant for user registration

use axum::{extract::State, response::Redirect, Form};

use crate::{
    auth::{
        principal::NoAuthPrincipal, session::SessionWriteRepository,
        session_providers::cookie::SetCookieSession,
    },
    database::WriteRepository,
    domain, validation, AppState,
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
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or(LoginError::Unknown)?;

    if !validation::is_email(&details.email) {
        return Err(LoginError::InvalidCredentials);
    }

    let user = domain::users::find_with_password(&mut repository, &details.email)
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

    let session = repository
        .create_session(user.id)
        .await
        .ok_or(LoginError::Unknown)?;

    repository
        .commit()
        .await
        .ok_or(LoginError::Unknown)?;
    Ok((SetCookieSession(session), Redirect::to("/")))
}
