//! Implements user registration into local database

use axum::{routing, Router};

use crate::AppState;

mod api;
mod error;
mod page;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", routing::get(page::handler))
        .route("/register", routing::post(api::handler))
}
