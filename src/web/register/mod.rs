//! This implements the action of registering a user.

use axum::{routing, Router};

use super::_core::State;

mod action;
mod responder;

pub fn router() -> Router<State> {
    Router::new().route("/register", routing::post(action::invoke))
}
