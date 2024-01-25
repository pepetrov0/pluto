//! Implements the registration page

use askama::Template;
use axum::extract::Query;

use crate::{auth::principal::NoAuthPrincipal, templates::HtmlTemplate};

use super::error::RegistrationError;

#[derive(serde::Deserialize)]
pub struct RegisterQuery {
    pub error: Option<RegistrationError>,
}

#[derive(Template, Debug, Default)]
#[template(path = "auth/local/register.html")]
pub struct RegisterPage {
    pub error: Option<RegistrationError>,
    pub timezones: Vec<String>,
}

pub async fn handler(
    _: NoAuthPrincipal,
    Query(query): Query<RegisterQuery>,
) -> HtmlTemplate<RegisterPage> {
    let timezones = chrono_tz::TZ_VARIANTS
        .iter()
        .map(|v| v.name().to_owned())
        .collect();
    HtmlTemplate(RegisterPage {
        error: query.error,
        timezones,
    })
}

mod filters {
    pub fn under_to_space<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace('_', " "))
    }

    pub fn slash_to_pipe<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace('/', " | "))
    }
}
