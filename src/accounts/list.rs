//! Implements accounts list page

use askama::Template;
use axum::{
    extract::{Query, State},
    routing, Router,
};

use crate::{
    accounts::component::Account, auth::principal::AuthPrincipal, templates::HtmlTemplate,
    user::{User, UserRepository}, AppState,
};

use super::{component::AccountRepository, ownership::AccountOwnershipRepository};

type AccountBundle = (Account, Vec<User>);

#[derive(serde::Deserialize)]
pub struct AccountsListQuery {
    #[serde(default)]
    pub created: bool,
}

#[derive(Template, Debug, Clone)]
#[template(path = "accounts/list.html")]
struct AccountsListPage {
    pub created: bool,
    pub accounts: Option<Vec<AccountBundle>>,
}

async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<AccountsListQuery>,
    State(mut state): State<AppState>,
) -> Result<HtmlTemplate<AccountsListPage>, HtmlTemplate<AccountsListPage>> {
    let construct_error = || {
        HtmlTemplate(AccountsListPage {
            created: query.created,
            accounts: None,
        })
    };

    // fetch all account ownerships
    let ownerships = state
        .database
        .list_account_ownerships_by_user(&user.id)
        .await
        .ok_or_else(construct_error)?;

    // fetch all accounts that are owned
    let accounts_owned = ownerships.into_iter().map(|v| v.account).collect();
    let accounts_owned = state
        .database
        .list_accounts_by_ids(accounts_owned)
        .await
        .ok_or_else(construct_error)?;

    // fetch all ownerships
    let ownerships = accounts_owned.iter().cloned().map(|v| v.id).collect();
    let ownerships = state
        .database
        .list_account_ownerships_by_accounts(ownerships)
        .await
        .ok_or_else(construct_error)?;

    // fetch all users
    let users = ownerships.iter().cloned().map(|v| v.usr).collect();
    let users = state
        .database
        .list_users_by_ids(users)
        .await
        .ok_or_else(construct_error)?;

    // aggregate all data
    let accounts_owned = accounts_owned
        .into_iter()
        .map(|v| {
            let owners = ownerships
                .iter()
                .filter(|&o| o.account == v.id)
                .flat_map(|v| users.iter().find(|&u| v.usr == u.id).cloned())
                .collect();

            (v, owners)
        })
        .collect();

    let page = AccountsListPage {
        created: query.created,
        accounts: Some(accounts_owned),
    };
    Ok(HtmlTemplate(page))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/accounts", routing::get(handler))
}
