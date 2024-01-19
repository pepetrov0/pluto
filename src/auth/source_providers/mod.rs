use axum::Router;

use crate::AppState;

mod email_password;

pub fn router() -> Router<AppState> {
    Router::new().merge(email_password::router())
}
