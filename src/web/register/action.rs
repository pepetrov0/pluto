use axum::response::Response;

use crate::web::_core::{Hx, Locale};

pub async fn invoke(locale: Locale, hx: Hx) -> Response {
    super::responder::invoke(locale, hx).await
}
