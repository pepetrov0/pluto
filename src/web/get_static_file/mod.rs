use axum::{routing, Router};

use super::_core::State;

mod action;
mod core;
mod responder;

pub use core::url;

pub fn router() -> Router<State> {
    Router::new().route("/static/:path", routing::get(action::invoke))
}
