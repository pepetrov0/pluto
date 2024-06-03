use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    domain::{registration::RegistrationError, Session},
    web::{
        _components::{
            organisms::{self, RegisterData},
            pages,
        },
        _core::{CreateAuth, Hx, Locale},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    data: RegisterData,
    result: Result<Session, RegistrationError>,
) -> Response {
    match result {
        Ok(session) => (CreateAuth(session), Redirect::to("/")).into_response(),
        Err(error) if hx.request => {
            organisms::register(locale.as_str(), Some(data), Some(error)).into_response()
        }
        Err(error) => pages::register(locale.as_str(), Some(data), Some(error)).into_response(),
    }
}
