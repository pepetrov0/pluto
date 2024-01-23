//! Implements the list page where all assets are listed

use askama::Template;
use axum::{extract::State, http::StatusCode, routing, Router};

use crate::{auth::principal::AuthPrincipal, templates::HtmlTemplate, AppState};

use super::component::{Asset, AssetType};

#[derive(Template, Debug, Clone)]
#[template(path = "assets/list.html")]
struct AssetsListPage {
    pub currencies_error: bool,
    pub currencies: Vec<Asset>,
}

async fn handler(
    _: AuthPrincipal,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<AssetsListPage>, StatusCode> {
    let assets = state.asset_repository.list_assets().await;

    let page = AssetsListPage {
        currencies_error: assets.is_none(),
        currencies: assets
            .unwrap_or_default()
            .into_iter()
            .filter(|v| v.atype == AssetType::Currency)
            .collect(),
    };
    Ok(HtmlTemplate(page))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/assets", routing::get(handler))
}