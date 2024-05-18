//! This module implements the action of showing the dashboard to the user.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route("/", routing::get(action::invoke))
}
