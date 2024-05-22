//! Implements dynamic validation of register form.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route("/register/validate", routing::post(action::invoke))
}
