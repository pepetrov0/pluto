use axum::Router;

use crate::AppState;

mod creation;
mod list;
pub mod ownership;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(list::router())
        .merge(creation::router())
}
