use axum::response::Response;

use crate::web::_core::{Auth, Locale};

pub async fn invoke(locale: Locale, auth: Option<Auth>) -> Response {
    super::responder::invoke(locale.as_str(), auth.is_some()).await
}
