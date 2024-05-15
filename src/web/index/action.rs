use axum::response::Response;

use crate::web::_core::Locale;

pub async fn invoke(locale: Locale) -> Response {
    super::responder::invoke(locale)
}
