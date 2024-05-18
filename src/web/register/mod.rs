//! This implements the action of registering a user.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

#[derive(Debug, serde::Deserialize)]
struct Arguments {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub fn router() -> Router<GlobalState> {
    Router::new().route("/register", routing::post(action::invoke))
}
