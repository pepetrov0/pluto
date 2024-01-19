//! Implements user registration into local database

use axum::{routing, Router};

use crate::{compression, AppState};

mod api;
mod error;
mod page;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/login",
            routing::get(page::handler).layer(compression::default()),
        )
        .route("/login", routing::post(api::handler))
}
