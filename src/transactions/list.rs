use askama::Template;
use axum::{
    extract::{Query, State},
    routing, Router,
};
use chrono_tz::Tz;
use either::Either;
use itertools::Itertools;

use crate::{
    accounts::{
        component::AccountReadonlyRepository, ownership::AccountOwnershipReadonlyRepository,
    },
    auth::principal::AuthPrincipal,
    core::database::ReadonlyRepository,
    core::web::templates::HtmlTemplate,
    domain, AppState, DEFAULT_PAGE_SIZE, PAGE_SIZE_LIMITS,
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
    State(state): State<AppState>,
) -> Result<HtmlTemplate<TransactionsList>, HtmlTemplate<TransactionsList>> {
    let construct_error = || HtmlTemplate(TransactionsList::default());
    let mut repository = ReadonlyRepository::from_pool(&state.database)
        .await
        .ok_or_else(construct_error)?;

    let tz = Tz::from_str_insensitive(&user.timezone).map_err(|_| construct_error())?;

    // all owned accounts for the user
    let owned_accounts: Vec<_> = repository
        .list_account_ownerships_by_user_or_account(&user.id)
        .await
        .ok_or_else(construct_error)?
        .into_iter()
        .map(|v| v.account)
        .collect();

    // number of transactions
    let number_of_transactions = repository
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
    let unsettled_transactions = repository
        .list_unsettled_transactions(&owned_accounts)
        .await
        .ok_or_else(construct_error)?;
    let settled_transactions = repository
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
    let assets = domain::assets::list_by_ids_or_tickers(&mut repository, &assets)
        .await
        .ok_or_else(construct_error)?;

    // accounts
    let accounts = all_transactions
        .iter()
        .flat_map(|v| [v.credit_account.clone(), v.debit_account.clone()])
        .collect();
    let accounts = repository
        .list_accounts_by_ids(accounts)
        .await
        .ok_or_else(construct_error)?;

    // account ownerships
    let ownerships = accounts.iter().map(|v| v.id.clone()).collect();
    let ownerships = repository
        .list_account_ownerships_by_users_or_accounts(ownerships)
        .await
        .ok_or_else(construct_error)?;

    // users
    let users = ownerships.iter().map(|v| v.usr.clone()).collect_vec();
    let users = domain::users::list_by_ids_or_emails(&mut repository, &users)
        .await
        .map_err(|_| construct_error())?;

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
