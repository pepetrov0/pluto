//! Implements an action to change a user's password.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route("/profile/change-password", routing::post(action::invoke))
}