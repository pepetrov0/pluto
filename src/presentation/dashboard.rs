use askama::Template;
use axum::{routing, Router};

use crate::{auth::principal::AuthPrincipal, AppState};

use super::core::{HtmlTemplate, IntoHtmlTemplate, IntoPage, Page};

#[derive(Template, Debug, Default)]
#[template(path = "dashboard.html")]
pub struct DashboardPage;

async fn handler(AuthPrincipal(_): AuthPrincipal) -> HtmlTemplate<Page<DashboardPage>> {
    DashboardPage
        .into_page("Dashboard".to_string())
        .into_html_template()
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(handler))
}
