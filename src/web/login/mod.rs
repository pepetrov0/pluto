//! This implements the action of logging in a user.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route("/login", routing::post(action::invoke))
}
