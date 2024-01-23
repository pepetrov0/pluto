//! Implements the assets feature

use crate::AppState;
use axum::Router;

pub mod component;

mod list;

pub fn router() -> Router<AppState> {
    Router::new().merge(list::router())
}
