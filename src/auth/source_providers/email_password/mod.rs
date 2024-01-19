//! Enables local authorization and authentication

use axum::Router;

use crate::AppState;

mod login;
mod register;

pub fn router() -> Router<AppState> {
    Router::new().merge(login::router()).merge(register::router())
}
