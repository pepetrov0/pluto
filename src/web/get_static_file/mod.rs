//! This module implements the action of retrieving a static file.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route("/static/:path", routing::get(action::invoke))
}
