//! Implements the list page where all assets are listed

use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing, Router,
};

use crate::{auth::principal::AuthPrincipal, templates::HtmlTemplate, AppState};

use super::component::{Asset, AssetType};

#[derive(serde::Deserialize)]
pub struct AssetsListQuery {
    #[serde(default)]
    pub created: bool,
}

#[derive(Template, Debug, Clone)]
#[template(path = "assets/list.html")]
struct AssetsListPage {
    pub created: bool,
    pub currencies: Option<Vec<Asset>>,
}

async fn handler(
    _: AuthPrincipal,
    Query(query): Query<AssetsListQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<AssetsListPage>, StatusCode> {
    let assets = state.asset_repository.list_assets().await;

    let page = AssetsListPage {
        created: query.created,
        currencies: assets.map(|v| {
            v.into_iter()
                .filter(|v| v.atype == AssetType::Currency)
                .collect()
        }),
    };
    Ok(HtmlTemplate(page))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/assets", routing::get(handler))
}
