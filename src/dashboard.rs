use axum::{
    response::{IntoResponse, Response},
    routing, Router,
};

use crate::{auth::principal::AuthPrincipal, AppState};

async fn handler(AuthPrincipal(user): AuthPrincipal) -> Response {
    format!("{:?}", user).into_response()
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(handler))
}
