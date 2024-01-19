//! Implements the API relevant for user registration

use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};

use crate::{auth::session_providers::cookie::SetCookieSession, validation, AppState};

use super::error::RegistrationError;

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub async fn handler(State(state): State<AppState>, Form(details): Form<RegisterForm>) -> Response {
    if validation::is_email(&details.email) {
        return RegistrationError::InvalidEmail.into_response();
    }

    if details.password != details.confirm_password {
        return RegistrationError::PasswordsMismatch.into_response();
    }

    if state
        .user_repository
        .find_user(&details.email)
        .await
        .is_some()
    {
        return RegistrationError::EmailTaken.into_response();
    }

    let hash = match state.password_hasher.hash(details.password) {
        Some(hash) => hash,
        None => return RegistrationError::Unknown.into_response(),
    };

    match state
        .user_repository
        .create_user(details.email, Some(hash))
        .await
    {
        // create a session and redirect to /
        Some(user) => match state.session_repository.create_session(user.id).await {
            Some(session) => (SetCookieSession(session), Redirect::to("/")).into_response(),
            None => Redirect::to("/").into_response(),
        },
        None => RegistrationError::Unknown.into_response(),
    }
}
