use askama::Template;
use axum::extract::{Query, State};

use crate::{
    auth::principal::AuthPrincipal,
    core::database::WriteRepository,
    domain::csrf_tokens::{self, CsrfToken},
    presentation::core::{HtmlTemplate, IntoHtmlTemplate, IntoPage, Page},
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
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewAccountQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<Page<NewAccountPage>>, HtmlTemplate<Page<NewAccountPage>>> {
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or_else(|| {
            NewAccountPage::default()
                .into_page("New account".to_string())
                .into_html_template()
        })?;

    // create csrf token
    let csrf_token = csrf_tokens::create(&mut repository, &user, super::CSRF_TOKEN_USAGE)
        .await
        .ok();

    let page = NewAccountPage {
        csrf_token,
        error: query.error,
    };

    repository.commit().await.ok_or_else(|| {
        NewAccountPage::default()
            .into_page("New account".to_string())
            .into_html_template()
    })?;
    Ok(page
        .into_page("New account".to_string())
        .into_html_template())
}
