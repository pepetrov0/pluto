use axum::{routing, Router};

use super::_core::State;

mod action;
mod responder;

pub fn router() -> Router<State> {
    Router::new().route("/", routing::get(action::invoke))
}
