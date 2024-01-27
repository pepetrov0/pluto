//! Implements transactions

use axum::Router;

use crate::AppState;

pub mod component;
pub mod entries;

mod creation;
mod list;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(list::router())
        .merge(creation::router())
}
