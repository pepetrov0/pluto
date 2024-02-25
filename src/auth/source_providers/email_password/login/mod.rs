use axum::{routing, Router};

use crate::AppState;

mod api;
mod error;
mod page;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", routing::get(page::handler))
        .route("/login", routing::post(api::handler))
}
