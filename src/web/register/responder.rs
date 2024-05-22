use axum::response::{IntoResponse, Response};

use crate::{
    domain::{registration::RegistrationError, sessions::Session},
    web::{
        _components::{organisms::RegisterFormData, pages},
        _core::{CreateAuth, Hx, Locale, Redirect},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    data: RegisterFormData,
    result: Result<Session, RegistrationError>,
) -> Response {
    match result {
        Ok(session) => (CreateAuth(session), Redirect::see_other(hx, "/")).into_response(),
        Err(error) => pages::register(locale.as_str(), Some(data), Some(error)).into_response(),
    }
}
