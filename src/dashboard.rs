use askama::Template;
use axum::{routing, Router};

use crate::{auth::principal::AuthPrincipal, templates::{HtmlTemplate, Navigation}, AppState};

#[derive(Template, Debug, Default)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {
    pub navigation: Navigation
}

async fn handler(AuthPrincipal(_): AuthPrincipal) -> HtmlTemplate<DashboardPage> {
    HtmlTemplate(DashboardPage { navigation: Navigation::default() })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(handler))
}
