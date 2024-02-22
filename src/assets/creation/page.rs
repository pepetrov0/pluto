//! Implements asset creation page

use askama::Template;
use axum::extract::{Query, State};

use crate::{
    auth::principal::AuthPrincipal,
    csrf_tokens::{CsrfToken, CsrfTokenRepository},
    database::WriteRepository,
    templates::HtmlTemplate,
    AppState,
};

use super::error::AssetCreationError;

#[derive(serde::Deserialize)]
pub struct NewAssetQuery {
    pub error: Option<AssetCreationError>,
}

#[derive(Template, Default)]
#[template(path = "assets/creation.html")]
pub struct NewAssetPage {
    pub csrf_token: Option<CsrfToken>,
    pub error: Option<AssetCreationError>,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewAssetQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<NewAssetPage>, HtmlTemplate<NewAssetPage>> {
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or_else(|| HtmlTemplate(NewAssetPage::default()))?;

    // create csrf token
    let csrf_token = repository
        .create_csrf_token(&user.id, super::CSRF_TOKEN_USAGE)
        .await;

    let page = HtmlTemplate(NewAssetPage {
        csrf_token,
        error: query.error,
    });

    repository
        .commit()
        .await
        .ok_or_else(|| HtmlTemplate(NewAssetPage::default()))?;
    Ok(page)
}
