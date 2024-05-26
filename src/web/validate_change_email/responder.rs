use axum::response::{IntoResponse, Response};

use crate::{
    domain::change_email::ChangeEmailError,
    web::{
        _components::organisms::{change_email_form, ChangeEmailFormData},
        _core::Locale,
    },
};

pub async fn invoke(
    locale: Locale,
    data: ChangeEmailFormData,
    error: Option<ChangeEmailError>,
) -> Response {
    change_email_form(locale.as_str(), Some(data), error).into_response()
}
