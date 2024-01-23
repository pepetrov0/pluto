use askama::Template;
use axum::extract::State;

use crate::{assets::component::Asset, auth::principal::AuthPrincipal, templates::HtmlTemplate, AppState};

#[derive(Template, Debug)]
#[template(path = "accounts/creation.html")]
pub struct NewAccountPage {
    pub assets: Option<Vec<Asset>>
}

pub async fn handler(_: AuthPrincipal, State(state): State<AppState>) -> HtmlTemplate<NewAccountPage> {
    let assets = state.asset_repository.list_assets().await;
    HtmlTemplate(NewAccountPage { assets })
}
