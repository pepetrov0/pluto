use askama::Template;
use axum::extract::{Query, State};

use crate::{
    assets::component::{Asset, AssetRepository},
    auth::principal::AuthPrincipal,
    csrf_tokens::{CsrfToken, CsrfTokenRepository},
    templates::HtmlTemplate,
    AppState,
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
    State(mut state): State<AppState>,
) -> HtmlTemplate<NewAccountPage> {
    // create csrf token
    let csrf_token = state
        .database
        .create_csrf_token(&user.id, super::CSRF_TOKEN_USAGE)
        .await;

    // fetch currencies
    let currencies = state.database.list_assets().await;

    HtmlTemplate(NewAccountPage {
        csrf_token,
        error: query.error,
        assets: currencies,
    })
}
