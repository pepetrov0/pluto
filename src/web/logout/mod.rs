//! Implementation of logout allowing users to delete their sessions.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route("/logout", routing::any(action::invoke))
}
