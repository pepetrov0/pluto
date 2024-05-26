//! Implements an action to validate a 'change email' form.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route(
        "/profile/change-email/validate",
        routing::post(action::invoke),
    )
}
