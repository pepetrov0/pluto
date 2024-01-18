//! Enables local authorization and authentication

use axum::Router;

use crate::AppState;

mod register;

pub fn router() -> Router<AppState> {
    Router::new().merge(register::router())
}
