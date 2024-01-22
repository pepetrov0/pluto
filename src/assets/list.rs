//! Implements the list page where all assets are listed

use askama::Template;
use axum::extract::State;

use crate::{templates::HtmlTemplate, AppState, auth::principal::AuthPrincipal};

use super::component::{Asset, AssetType};

#[derive(Template, Debug, Clone)]
#[template(path = "assets/list.html")]
pub struct AssetsListPage {
    pub currencies: Vec<Asset>,
}

pub async fn handler(_: AuthPrincipal, State(state): State<AppState>) -> HtmlTemplate<AssetsListPage> {
    let assets = state.asset_repository.list_assets().await.unwrap(); // FIXME: implement error handling
    HtmlTemplate(AssetsListPage {
        currencies: assets
            .iter()
            .filter(|v| v.atype == AssetType::Currency)
            .cloned()
            .collect(),
    })
}
