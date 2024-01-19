//! Implements the registration page

use askama::Template;
use axum::extract::Query;

use crate::{templates::HtmlTemplate, auth::principal::NoAuthPrincipal};

use super::error::RegistrationError;

#[derive(serde::Deserialize)]
pub struct RegisterQuery {
    pub error: Option<RegistrationError>,
}

#[derive(Template, Debug, Default)]
#[template(path = "auth/local/register.html")]
pub struct RegisterPage {
    pub error: Option<RegistrationError>,
}

pub async fn handler(_: NoAuthPrincipal, Query(query): Query<RegisterQuery>) -> HtmlTemplate<RegisterPage> {
    HtmlTemplate(RegisterPage { error: query.error })
}
