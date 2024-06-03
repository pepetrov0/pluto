use axum::response::{IntoResponse, Response};

use crate::{
    domain::change_email::ChangeEmailError,
    web::{
        _components::organisms::{change_email, ChangeEmailData},
        _core::Locale,
    },
};

pub async fn invoke(
    locale: Locale,
    data: ChangeEmailData,
    error: Option<ChangeEmailError>,
) -> Response {
    change_email(locale.as_str(), Some(data), error).into_response()
}
