use askama::Template;
use axum::extract::{Query, State};

use crate::{
    auth::principal::AuthPrincipal,
    database::WriteRepository,
    domain::csrf_tokens::{self, CsrfToken},
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
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewAccountQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<NewAccountPage>, HtmlTemplate<NewAccountPage>> {
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or_else(|| HtmlTemplate(NewAccountPage::default()))?;

    // create csrf token
    let csrf_token = csrf_tokens::create(&mut repository, &user, super::CSRF_TOKEN_USAGE)
        .await
        .ok();

    let page = HtmlTemplate(NewAccountPage {
        csrf_token,
        error: query.error,
    });

    repository
        .commit()
        .await
        .ok_or_else(|| HtmlTemplate(NewAccountPage::default()))?;
    Ok(page)
}
