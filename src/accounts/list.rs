use askama::Template;
use axum::{
    extract::{Query, State},
    routing, Router,
};
use itertools::Itertools;

use crate::{
    auth::principal::AuthPrincipal,
    core::{
        database::ReadonlyRepository,
        web::templates::{HtmlTemplate, IntoHtmlTemplate},
    },
    domain::{self, accounts::Account, users::User},
    presentation::core::{IntoPage, Page},
    AppState,
};

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
    pub principal: User,
}

async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<AccountsListQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<Page<AccountsListPage>>, HtmlTemplate<Page<AccountsListPage>>> {
    let construct_error = || {
        AccountsListPage {
            created: query.created,
            accounts: None,
            principal: user.clone(),
        }
        .into_page("Accounts".to_string())
        .into_html_template()
    };
    let mut repository = ReadonlyRepository::from_pool(&state.database)
        .await
        .ok_or_else(construct_error)?;

    // fetch all account ownerships
    let ownerships =
        domain::accounts_ownerships::list_by_user_or_account(&mut repository, &user.id)
            .await
            .map_err(|_| construct_error())?;

    // fetch all accounts that are owned
    let accounts_owned = ownerships.into_iter().map(|v| v.account).collect_vec();
    let accounts_owned = domain::accounts::list_by_ids(&mut repository, &accounts_owned)
        .await
        .map_err(|_| construct_error())?;

    // fetch all ownerships
    let ownerships = accounts_owned.iter().cloned().map(|v| v.id).collect_vec();
    let ownerships =
        domain::accounts_ownerships::list_by_users_or_accounts(&mut repository, &ownerships)
            .await
            .map_err(|_| construct_error())?;

    // fetch all users
    let users = ownerships.iter().cloned().map(|v| v.usr).collect_vec();
    let users = domain::users::list_by_ids_or_emails(&mut repository, &users)
        .await
        .map_err(|_| construct_error())?;

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
        principal: user,
    }
    .into_page("Accounts".to_string())
    .into_html_template();
    Ok(page)
}

pub fn router() -> Router<AppState> {
    Router::new().route("/accounts", routing::get(handler))
}
