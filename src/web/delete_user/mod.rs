//! A action to delete a user.

use axum::{routing::post, Router};

use super::_core::GlobalState;

mod action;
mod responder;

pub fn router() -> Router<GlobalState> {
    Router::new().route("/profile/delete", post(action::invoke))
}
