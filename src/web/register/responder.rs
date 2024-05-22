use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    domain::{registration::RegistrationError, sessions::Session},
    web::{
        _components::{organisms::RegisterFormData, pages},
        _core::{CreateAuth, Locale},
    },
};

pub async fn invoke(
    locale: Locale,
    data: RegisterFormData,
    result: Result<Session, RegistrationError>,
) -> Response {
    match result {
        Ok(session) => (CreateAuth(session), Redirect::to("/")).into_response(),
        Err(error) => pages::register(locale.as_str(), Some(data), Some(error)).into_response(),
    }
}
