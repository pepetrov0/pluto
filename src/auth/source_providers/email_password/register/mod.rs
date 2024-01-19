//! Enables the users to register locally

use axum::{routing, Router};

use crate::{compression, AppState};

mod api;
mod error;
mod page;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/register",
            routing::get(page::handler).layer(compression::default()),
        )
        .route("/register", routing::post(api::handler))
}
