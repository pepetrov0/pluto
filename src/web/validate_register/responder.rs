use axum::response::{IntoResponse, Response};

use crate::{
    domain::registration::RegistrationError,
    web::{
        _components::organisms::{self, RegisterData},
        _core::Locale,
    },
};

pub async fn invoke(
    locale: Locale,
    data: RegisterData,
    error: Option<RegistrationError>,
) -> Response {
    organisms::register(locale.as_str(), Some(data), error).into_response()
}
