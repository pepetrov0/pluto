//! Implements the accounts feature

use axum::Router;

use crate::AppState;

pub mod component;
mod list;
mod creation;

pub fn router() -> Router<AppState> {
    Router::new().merge(list::router()).merge(creation::router())
}
