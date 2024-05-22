use axum::response::Response;

use crate::web::_core::Hx;

pub async fn invoke(hx: Hx) -> Response {
    super::responder::invoke(hx).await
}
