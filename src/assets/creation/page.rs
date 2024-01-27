//! Implements asset creation page

use askama::Template;
use axum::extract::{Query, State};

use crate::{
    auth::principal::AuthPrincipal, csrf_tokens::CsrfToken, templates::HtmlTemplate, AppState,
};

use super::error::AssetCreationError;

#[derive(serde::Deserialize)]
pub struct NewAssetQuery {
    pub error: Option<AssetCreationError>,
}

#[derive(Template)]
#[template(path = "assets/creation.html")]
pub struct NewAssetPage {
    pub csrf_token: Option<CsrfToken>,
    pub error: Option<AssetCreationError>,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewAssetQuery>,
    State(state): State<AppState>,
) -> HtmlTemplate<NewAssetPage> {
    // create csrf token
    let csrf_token = state
        .csrf_token_repository
        .create_csrf_token(user.id.clone(), super::CSRF_TOKEN_USAGE)
        .await;

    HtmlTemplate(NewAssetPage {
        csrf_token,
        error: query.error,
    })
}
