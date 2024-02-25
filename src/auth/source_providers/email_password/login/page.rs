//! Implements the registration page

use askama::Template;
use axum::extract::Query;

use crate::{auth::principal::NoAuthPrincipal, core::web::templates::HtmlTemplate};

use super::error::LoginError;

#[derive(serde::Deserialize)]
pub struct LoginQuery {
    pub error: Option<LoginError>,
}

#[derive(Template, Debug, Default)]
#[template(path = "auth/local/login.html")]
pub struct LoginPage {
    pub error: Option<LoginError>,
}

pub async fn handler(
    _: NoAuthPrincipal,
    Query(query): Query<LoginQuery>,
) -> HtmlTemplate<LoginPage> {
    HtmlTemplate(LoginPage { error: query.error })
}
