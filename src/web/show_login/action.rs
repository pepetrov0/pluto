use axum::response::Response;

use crate::web::_core::{Auth, Hx, Locale};

#[tracing::instrument]
pub async fn invoke(locale: Locale, hx: Hx, auth: Option<Auth>) -> Response {
    super::responder::invoke(locale, hx, auth.is_some()).await
}
