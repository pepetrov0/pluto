use axum::{Router, routing};
use crate::AppState;


pub mod component;

mod list_page;

pub fn router() -> Router<AppState> {
    Router::new().route("/assets", routing::get(list_page::handler))
}