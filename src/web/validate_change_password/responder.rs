use axum::response::{IntoResponse, Response};

use crate::{
    domain::change_password::ChangePasswordError,
    web::{
        _components::organisms::{self, ChangePasswordData},
        _core::Locale,
    },
};

pub async fn invoke(
    locale: Locale,
    data: ChangePasswordData,
    error: Option<ChangePasswordError>,
) -> Response {
    organisms::change_password(locale.as_str(), Some(data), error).into_response()
}
