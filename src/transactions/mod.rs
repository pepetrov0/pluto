//! Implements transactions

use axum::Router;

use crate::AppState;

pub mod component;
pub mod entries;

mod list;

pub fn router() -> Router<AppState> {
    Router::new().merge(list::router())
}
