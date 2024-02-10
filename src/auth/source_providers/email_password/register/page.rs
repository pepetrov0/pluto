//! Implements the registration page

use askama::Template;
use axum::extract::{Query, State};

use crate::{
    assets::component::{Asset, AssetReadonlyRepository},
    auth::principal::NoAuthPrincipal,
    templates::HtmlTemplate,
    AppState,
};

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
    pub assets: Option<Vec<Asset>>,
}

pub async fn handler(
    _: NoAuthPrincipal,
    Query(query): Query<RegisterQuery>,
    State(mut state): State<AppState>,
) -> HtmlTemplate<RegisterPage> {
    let timezones = chrono_tz::TZ_VARIANTS
        .iter()
        .map(|v| v.name().to_owned())
        .collect();

    let assets = state.database.list_assets().await;

    HtmlTemplate(RegisterPage {
        error: query.error,
        timezones,
        assets,
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
