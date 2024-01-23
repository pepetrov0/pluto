//! Implements accounts list page

use askama::Template;
use axum::{routing, Router};

use crate::{auth::principal::AuthPrincipal, templates::HtmlTemplate, AppState};

#[derive(Template, Debug, Clone)]
#[template(path = "accounts/list.html")]
struct AccountsListPage {
}

async fn handler(_: AuthPrincipal) -> HtmlTemplate<AccountsListPage> {
    HtmlTemplate(AccountsListPage {})
}

pub fn router() -> Router<AppState> {
    Router::new().route("/accounts", routing::get(handler))
}