use axum::response::Response;

use crate::web::_core::{Auth, Locale};

#[tracing::instrument]
pub async fn invoke(locale: Locale, auth: Auth) -> Response {
    super::responder::invoke(locale, auth)
}
