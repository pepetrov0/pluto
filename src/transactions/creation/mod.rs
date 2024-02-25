use axum::{routing, Router};

use crate::AppState;

mod api;
mod error;
mod page;

pub(super) const CSRF_TOKEN_USAGE: &str = "new-transaction";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/new-transaction", routing::get(page::handler))
        .route("/new-transaction", routing::post(api::handler))
}
