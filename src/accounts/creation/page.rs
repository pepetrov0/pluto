use askama::Template;
use axum::extract::{Query, State};

use crate::{
    assets::component::Asset, auth::principal::AuthPrincipal, csrf_tokens::CsrfToken,
    templates::HtmlTemplate, AppState,
};

use super::error::AccountCreationError;

#[derive(serde::Deserialize)]
pub struct NewAccountQuery {
    pub error: Option<AccountCreationError>,
}

#[derive(Template, Debug)]
#[template(path = "accounts/creation.html")]
pub struct NewAccountPage {
    pub csrf_token: Option<CsrfToken>,
    pub error: Option<AccountCreationError>,
    pub assets: Option<Vec<Asset>>,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewAccountQuery>,
    State(state): State<AppState>,
) -> HtmlTemplate<NewAccountPage> {
    let csrf_token = state
        .csrf_token_repository
        .create_csrf_token(user.id.clone(), super::CSRF_TOKEN_USAGE)
        .await;
    let assets = state.asset_repository.list_assets().await;

    HtmlTemplate(NewAccountPage {
        csrf_token,
        error: query.error,
        assets,
    })
}
