//! Implements the list page for transactions

use askama::Template;
use axum::{
    extract::{Query, State},
    routing, Router,
};
use chrono_tz::Tz;
use either::Either;

use crate::{
    accounts::{
        component::AccountReadonlyRepository, ownership::AccountOwnershipReadonlyRepository,
    },
    assets::component::AssetReadonlyRepository,
    auth::principal::AuthPrincipal,
    templates::HtmlTemplate,
    users::UserReadonlyRepository,
    AppState, DEFAULT_PAGE_SIZE, PAGE_SIZE_LIMITS,
};

use super::{component::TransactionReadonlyRepository, models::ResolvedTransaction};

#[derive(serde::Deserialize)]
pub struct AccountsListQuery {
    #[serde(default)]
    pub created: bool,
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub size: Option<i64>,
}

#[derive(Template, Debug, Default)]
#[template(path = "transactions/list.html")]
struct TransactionsList {
    pub created: bool,
    pub unsettled_transactions: Option<Vec<ResolvedTransaction>>,
    pub settled_transactions: Option<Vec<ResolvedTransaction>>,
}

async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<AccountsListQuery>,
    State(mut state): State<AppState>,
) -> Result<HtmlTemplate<TransactionsList>, HtmlTemplate<TransactionsList>> {
    let construct_error = || HtmlTemplate(TransactionsList::default());

    let tz = Tz::from_str_insensitive(&user.timezone).map_err(|_| construct_error())?;

    // all owned accounts for the user
    let owned_accounts: Vec<_> = state
        .database
        .list_account_ownerships_by_user_or_account(&user.id)
        .await
        .ok_or_else(construct_error)?
        .into_iter()
        .map(|v| v.account)
        .collect();

    // number of transactions
    let number_of_transactions = state
        .database
        .count_settled_transactions(&owned_accounts)
        .await
        .ok_or_else(construct_error)?;

    // page params
    let page_size = query
        .size
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .clamp(PAGE_SIZE_LIMITS.0, PAGE_SIZE_LIMITS.1);
    let num_pages = ((number_of_transactions as f64 / page_size as f64).ceil() as i64).max(1);
    let page_offset = (query.page - 1).clamp(0, num_pages - 1) * page_size;

    // transactions
    let unsettled_transactions = state
        .database
        .list_unsettled_transactions(&owned_accounts)
        .await
        .ok_or_else(construct_error)?;
    let settled_transactions = state
        .database
        .list_settled_transactions(page_offset, page_size, &owned_accounts)
        .await
        .ok_or_else(construct_error)?;
    let all_transactions: Vec<_> = unsettled_transactions
        .iter()
        .chain(settled_transactions.iter())
        .cloned()
        .collect();

    // assets
    let assets: Vec<_> = all_transactions
        .iter()
        .flat_map(|v| [v.credit_asset.clone(), v.debit_asset.clone()])
        .collect();
    let assets = state
        .database
        .list_assets_by_ids(assets)
        .await
        .ok_or_else(construct_error)?;

    // accounts
    let accounts = all_transactions
        .iter()
        .flat_map(|v| [v.credit_account.clone(), v.debit_account.clone()])
        .collect();
    let accounts = state
        .database
        .list_accounts_by_ids(accounts)
        .await
        .ok_or_else(construct_error)?;

    // account ownerships
    let ownerships = accounts.iter().map(|v| v.id.clone()).collect();
    let ownerships = state
        .database
        .list_account_ownerships_by_users_or_accounts(ownerships)
        .await
        .ok_or_else(construct_error)?;

    // users
    let users = ownerships.iter().map(|v| v.usr.clone()).collect();
    let users = state
        .database
        .list_users_by_ids(users)
        .await
        .ok_or_else(construct_error)?;

    //  transactions
    let unsettled_transactions: Vec<_> = unsettled_transactions
        .into_iter()
        .filter_map(|v| v.into_resolved(&tz, &user, &users, &assets, &accounts, &ownerships))
        .collect();
    let settled_transactions: Vec<_> = settled_transactions
        .into_iter()
        .filter_map(|v| v.into_resolved(&tz, &user, &users, &assets, &accounts, &ownerships))
        .collect();

    let page = TransactionsList {
        created: query.created,
        unsettled_transactions: Some(unsettled_transactions),
        settled_transactions: Some(settled_transactions),
    };
    Ok(HtmlTemplate(page))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/transactions", routing::get(handler))
}
