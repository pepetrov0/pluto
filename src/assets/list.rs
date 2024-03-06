use askama::Template;
use axum::{
    extract::{Query, State},
    routing, Router,
};

use crate::{
    auth::principal::AuthPrincipal,
    core::database::ReadonlyRepository,
    domain::{
        self,
        assets::{Asset, AssetType},
        users::User,
    },
    presentation::core::{HtmlTemplate, IntoHtmlTemplate, IntoPage, Page},
    AppState,
};

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
    State(state): State<AppState>,
) -> HtmlTemplate<Page<AssetsListPage>> {
    let assets = match ReadonlyRepository::from_pool(&state.database).await {
        Some(mut repository) => domain::assets::list(&mut repository).await.ok(),
        None => None,
    };

    let page = AssetsListPage {
        created: query.created,
        currencies: assets.map(|v| {
            v.into_iter()
                .filter(|v| v.atype == AssetType::Currency)
                .collect()
        }),
        principal: user,
    };
    page.into_page("Currencies & Assets".to_string()).into_html_template()
}

pub fn router() -> Router<AppState> {
    Router::new().route("/assets", routing::get(handler))
}
