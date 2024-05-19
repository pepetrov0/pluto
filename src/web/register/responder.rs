use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie;

use crate::{
    domain::{registration::RegistrationError, sessions::Session},
    web::{
        _components::{
            organisms::{self, RegisterFormData},
            pages,
        },
        _core::{CreateAuth, Hx, Locale, Redirect},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    key: cookie::Key,
    data: RegisterFormData,
    result: Result<Session, RegistrationError>,
) -> Response {
    match result {
        Ok(session) => (CreateAuth(key, session.id), Redirect::see_other(hx, "/")).into_response(),
        Err(error) => match hx.request && !hx.boosted {
            true => {
                organisms::register_form(locale.as_str(), Some(data), Some(error)).into_response()
            }
            false => pages::register(locale.as_str(), Some(data), Some(error)).into_response(),
        },
    }
}
