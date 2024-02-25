use axum::Router;

use crate::AppState;

mod logout;
pub mod password_hasher;
pub mod principal;
pub(crate) mod session;
pub mod session_providers;
mod source_providers;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(source_providers::router())
        .merge(logout::router())
}
