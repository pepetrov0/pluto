//! Implements accounts list page

use askama::Template;
use axum::{
    extract::{Query, State},
    routing, Router,
};

use crate::{auth::principal::AuthPrincipal, templates::HtmlTemplate, AppState};

use super::component::Account;

#[derive(serde::Deserialize)]
pub struct AccountsListQuery {
    #[serde(default)]
    pub created: bool,
}

#[derive(Template, Debug, Clone)]
#[template(path = "accounts/list.html")]
struct AccountsListPage {
    pub created: bool,
    pub accounts: Option<Vec<Account>>,
}

async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<AccountsListQuery>,
    State(state): State<AppState>,
) -> HtmlTemplate<AccountsListPage> {
    let accounts_owned = state
        .account_ownership_repository
        .find_account_ownerships_by_user(&user.id)
        .await;

    let accounts = match accounts_owned {
        Some(ownerships) if ownerships.is_empty() => Some(vec![]),
        Some(ownerships) => {
            let accounts_owned = ownerships.into_iter().map(|v| v.account).collect();
            state
                .account_repository
                .find_all_accounts_by_ids(accounts_owned)
                .await
        }
        None => None,
    };

    HtmlTemplate(AccountsListPage {
        created: query.created,
        accounts,
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/accounts", routing::get(handler))
}
