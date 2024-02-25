use askama::Template;
use axum::extract::{Query, State};

use crate::{
    auth::principal::NoAuthPrincipal,
    core::database::ReadonlyRepository,
    core::web::templates::HtmlTemplate,
    domain::{self, assets::Asset},
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
    State(state): State<AppState>,
) -> Result<HtmlTemplate<RegisterPage>, HtmlTemplate<RegisterPage>> {
    let construct_error = || {
        HtmlTemplate(RegisterPage {
            error: query.error,
            timezones: vec![],
            assets: None,
        })
    };
    let mut repository = ReadonlyRepository::from_pool(&state.database)
        .await
        .ok_or_else(construct_error)?;

    let timezones = chrono_tz::TZ_VARIANTS
        .iter()
        .map(|v| v.name().to_owned())
        .collect();
    let assets = domain::assets::list(&mut repository)
        .await
        .map_err(|_| construct_error())?;

    let page = HtmlTemplate(RegisterPage {
        error: query.error,
        timezones,
        assets: Some(assets),
    });
    Ok(page)
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
