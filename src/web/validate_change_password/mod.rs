//! Implements dynamic validation of change password form.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route(
        "/profile/change-password/validate",
        routing::post(action::invoke),
    )
}
