use askama::Template;
use axum::extract::Query;

use crate::{
    auth::principal::NoAuthPrincipal,
    core::web::templates::{HtmlTemplate, IntoHtmlTemplate},
    presentation::core::{IntoPage, BlankPage},
};

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
) -> HtmlTemplate<BlankPage<LoginPage>> {
    LoginPage { error: query.error }
        .into_blank_page("Sign in".to_string())
        .into_html_template()
}
