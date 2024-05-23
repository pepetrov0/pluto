//! This module implements the action of showing the user their profile.

use axum::{routing, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route("/profile", routing::get(action::invoke))
}
