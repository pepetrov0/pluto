//! Implements the list page where all assets are listed

use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing, Router,
};

use crate::{auth::principal::AuthPrincipal, templates::HtmlTemplate, users::User, AppState};

use super::component::{Asset, AssetReadonlyRepository, AssetType};

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
    pub principal: User,
}

async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<AssetsListQuery>,
    State(mut state): State<AppState>,
) -> Result<HtmlTemplate<AssetsListPage>, StatusCode> {
    let assets = state.database.list_assets().await;

    let page = AssetsListPage {
        created: query.created,
        currencies: assets.map(|v| {
            v.into_iter()
                .filter(|v| v.atype == AssetType::Currency)
                .collect()
        }),
        principal: user,
    };
    Ok(HtmlTemplate(page))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/assets", routing::get(handler))
}
