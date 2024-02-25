use axum::{extract::State, response::Redirect, Form};

use crate::{
    auth::{principal::NoAuthPrincipal, session_providers::cookie::SetCookieSession},
    core::database::WriteRepository,
    domain::{self, sessions::SessionCreationError},
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
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or(LoginError::Unknown)?;

    if !validation::is_email(&details.email) {
        return Err(LoginError::InvalidCredentials);
    }

    let user = domain::users::find_with_password(&mut repository, &details.email)
        .await
        .ok()
        .flatten()
        .ok_or(LoginError::InvalidCredentials)?;

    let hash = match &user.password {
        Some(password) => password,
        None => return Err(LoginError::InvalidCredentials),
    };

    if !state
        .password_hasher
        .verify(details.password.as_bytes(), hash.as_str())
    {
        return Err(LoginError::InvalidCredentials);
    }

    let session = domain::sessions::create(&mut repository, &user.into())
        .await
        .map_err(LoginError::from)?;

    repository.commit().await.ok_or(LoginError::Unknown)?;
    Ok((SetCookieSession(session), Redirect::to("/")))
}

impl From<SessionCreationError> for LoginError {
    fn from(value: SessionCreationError) -> Self {
        match value {
            SessionCreationError::Unknown => Self::Unknown,
        }
    }
}
