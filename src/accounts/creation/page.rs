use askama::Template;
use axum::extract::{Query, State};

use crate::{
    assets::component::{Asset, AssetReadonlyRepository},
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

#[derive(Template, Debug, Default)]
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
) -> Result<HtmlTemplate<NewAccountPage>, HtmlTemplate<NewAccountPage>> {
    let mut tx = state
        .database
        .begin()
        .await
        .map_err(|_| HtmlTemplate(NewAccountPage::default()))?;

    // create csrf token
    let csrf_token = tx
        .create_csrf_token(&user.id, super::CSRF_TOKEN_USAGE)
        .await;

    // fetch currencies
    let currencies = tx.list_assets().await;

    let page = HtmlTemplate(NewAccountPage {
        csrf_token,
        error: query.error,
        assets: currencies,
    });

    tx.commit()
        .await
        .map_err(|_| HtmlTemplate(NewAccountPage::default()))?;
    Ok(page)
}
