//! Implements the dashboard page

use askama::Template;
use axum::{routing, Router};

use crate::{auth::principal::AuthPrincipal, core::web::templates::HtmlTemplate, AppState};

#[derive(Template, Debug, Default)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {}

async fn handler(AuthPrincipal(_): AuthPrincipal) -> HtmlTemplate<DashboardPage> {
    HtmlTemplate(DashboardPage {})
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(handler))
}
