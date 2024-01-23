//! Implements the accounts feature

use axum::{routing, Router};

use crate::AppState;

pub mod component;
mod list;

pub fn router() -> Router<AppState> {
    Router::new().route("/accounts", routing::get(list::handler))
}
