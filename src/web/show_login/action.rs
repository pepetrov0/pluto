use axum::response::Response;

use crate::web::_core::{Auth, Locale};

#[tracing::instrument]
pub async fn invoke(locale: Locale, auth: Option<Auth>) -> Response {
    super::responder::invoke(locale, auth.is_some()).await
}
