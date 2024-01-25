//! Implements the list page for transactions

use askama::Template;
use axum::{routing, Router};

use crate::{templates::HtmlTemplate, AppState};

#[derive(Template, Debug)]
#[template(path = "transactions/list.html")]
struct TransactionsList {}

async fn handler() -> HtmlTemplate<TransactionsList> {
    HtmlTemplate(TransactionsList {})
}

pub fn router() -> Router<AppState> {
    Router::new().route("/transactions", routing::get(handler))
}
