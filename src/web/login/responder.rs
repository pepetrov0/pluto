use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    domain::{authentication::AuthenticationError, sessions::Session},
    web::{
        _components::pages,
        _core::{CreateAuth, Locale},
    },
};

pub async fn invoke(locale: Locale, result: Result<Session, AuthenticationError>) -> Response {
    match result {
        Ok(session) => (CreateAuth(session), Redirect::to("/")).into_response(),
        Err(error) => pages::login(locale.as_str(), Some(error)).into_response(),
    }
}
