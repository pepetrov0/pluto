use axum::response::Response;

use crate::web::_core::{Hx, Auth, Locale};

#[tracing::instrument]
pub async fn invoke(locale: Locale, hx: Hx, auth: Option<Auth>) -> Response {
    super::responder::invoke(locale.as_str(), hx, auth.is_some()).await
}
