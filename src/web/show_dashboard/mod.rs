//! This module implements the action of showing the dashboard to the user.

use axum::{routing, Router};

use super::_core::State;

mod action;
mod responder;

pub fn router() -> Router<State> {
    Router::new().route("/", routing::get(action::invoke))
}
