use axum::response::{IntoResponse, Response};

use crate::{
    domain::registration::RegistrationError,
    web::{
        _components::organisms::{self, RegisterFormData},
        _core::Locale,
    },
};

pub async fn invoke(
    locale: Locale,
    data: RegisterFormData,
    error: Option<RegistrationError>,
) -> Response {
    organisms::register_form(locale.as_str(), Some(data), error).into_response()
}
