use axum::response::{IntoResponse, Response};

use crate::{
    domain::change_password::ChangePasswordError,
    web::{
        _components::organisms::{self, ChangePasswordFormData},
        _core::Locale,
    },
};

pub async fn invoke(
    locale: Locale,
    data: ChangePasswordFormData,
    error: Option<ChangePasswordError>,
) -> Response {
    organisms::change_password_form(locale.as_str(), Some(data), error).into_response()
}
