//! Implements the assets feature

use crate::AppState;
use axum::Router;

pub mod component;

mod creation;
mod list;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(list::router())
        .merge(creation::router())
}
