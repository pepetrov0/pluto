//! Implements the registration page

use askama::Template;
use axum::extract::Query;

use crate::{templates::HtmlTemplate, auth::principal::NoAuthPrincipal};

use super::error::LoginError;

#[derive(serde::Deserialize)]
pub struct RegisterQuery {
    pub error: Option<LoginError>,
}

#[derive(Template, Debug, Default)]
#[template(path = "auth/local/login.html")]
pub struct RegisterPage {
    pub error: Option<LoginError>,
}

pub async fn handler(_: NoAuthPrincipal, Query(query): Query<RegisterQuery>) -> HtmlTemplate<RegisterPage> {
    HtmlTemplate(RegisterPage { error: query.error })
}
