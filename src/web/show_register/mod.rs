//! This module implements the action of showing the register page to the user.

use axum::{routing, Router};

mod action;
mod responder;

pub fn router() -> Router<super::_core::GlobalState> {
    Router::new().route("/register", routing::get(action::invoke))
}
