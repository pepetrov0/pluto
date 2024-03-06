use askama::Template;
use axum::extract::{Query, State};

use crate::{
    auth::principal::AuthPrincipal,
    core::database::WriteRepository,
    domain::csrf_tokens::{self, CsrfToken},
    presentation::core::{HtmlTemplate, IntoHtmlTemplate, IntoPage, Page},
    AppState,
};

use super::error::AssetCreationError;

#[derive(serde::Deserialize)]
pub struct NewAssetQuery {
    pub error: Option<AssetCreationError>,
}

#[derive(Template, Default)]
#[template(path = "assets/creation.html")]
pub struct NewAssetPage {
    pub csrf_token: Option<CsrfToken>,
    pub error: Option<AssetCreationError>,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewAssetQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<Page<NewAssetPage>>, HtmlTemplate<Page<NewAssetPage>>> {
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or_else(|| {
            NewAssetPage::default()
                .into_page("New asset".to_string())
                .into_html_template()
        })?;

    // create csrf token
    let csrf_token = csrf_tokens::create(&mut repository, &user, super::CSRF_TOKEN_USAGE)
        .await
        .ok();

    let page = NewAssetPage {
        csrf_token,
        error: query.error,
    };

    repository.commit().await.ok_or_else(|| {
        NewAssetPage::default()
            .into_page("New asset".to_string())
            .into_html_template()
    })?;
    Ok(page.into_page("New asset".to_string()).into_html_template())
}
