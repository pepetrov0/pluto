//! Implements the accounts feature

use axum::Router;

use crate::AppState;

pub mod component;
pub mod ownership;
mod creation;
mod list;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(list::router())
        .merge(creation::router())
}
