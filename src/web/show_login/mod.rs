//! This module implements the action of showing the login page to the user.

use axum::{routing, Router};

mod action;
mod responder;

pub fn router() -> Router<super::_core::State> {
    Router::new().route("/login", routing::get(action::invoke))
}
