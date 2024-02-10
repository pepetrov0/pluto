//! Implements the list page for transactions

use askama::Template;
use axum::{
    extract::{Query, State},
    routing, Router,
};
use chrono_tz::Tz;
use either::Either;

use crate::{
    accounts::component::Account, assets::component::Asset, auth::principal::AuthPrincipal,
    templates::HtmlTemplate, user::User, AppState, DATE_TIME_FORMAT, DATE_TIME_FORMAT_NICE,
    DEFAULT_PAGE_SIZE, PAGE_SIZE_LIMITS,
};

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
    pub transactions: Option<Vec<TransactionBundle>>,
}

#[derive(Debug)]
struct TransactionBundle {
    pub note: String,
    pub credit: Either<Account, Vec<User>>,
    pub debit: Either<Account, Vec<User>>,
    pub credit_asset: Asset,
    pub debit_asset: Asset,
    pub credit_amount: f64,
    pub debit_amount: f64,
    pub credit_stamp: String,
    pub debit_stamp: String,
    pub credit_stamp_nice: String,
    pub debit_stamp_nice: String,
}

async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<AccountsListQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<TransactionsList>, HtmlTemplate<TransactionsList>> {
    let construct_error = || HtmlTemplate(TransactionsList::default());

    let tz = Tz::from_str_insensitive(&user.timezone).map_err(|_| construct_error())?;

    // all owned accounts for the user
    let owned_accounts: Vec<_> = state
        .account_ownership_repository
        .list_account_ownerships_by_user(&user.id)
        .await
        .ok_or_else(construct_error)?
        .into_iter()
        .map(|v| v.account)
        .collect();

    // number of transactions
    let number_of_transactions = state
        .transaction_repository
        .count_settled_transactions(owned_accounts.clone())
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
    let transactions = state
        .transaction_repository
        .list_settled_transactions(page_offset, page_size, owned_accounts)
        .await
        .ok_or_else(construct_error)?;

    // assets
    let assets: Vec<_> = transactions
        .iter()
        .flat_map(|v| [v.credit_asset.clone(), v.debit_asset.clone()])
        .collect();
    let assets = state
        .asset_repository
        .list_assets_by_ids(assets)
        .await
        .ok_or_else(construct_error)?;

    // accounts
    let accounts = transactions
        .iter()
        .flat_map(|v| [v.credit_account.clone(), v.debit_account.clone()])
        .collect();
    let accounts = state
        .account_repository
        .list_accounts_by_ids(accounts)
        .await
        .ok_or_else(construct_error)?;

    // account ownerships
    let ownerships = accounts.iter().map(|v| v.id.clone()).collect();
    let ownerships = state
        .account_ownership_repository
        .list_account_ownerships_by_accounts(ownerships)
        .await
        .ok_or_else(construct_error)?;

    // users
    let users = ownerships.iter().map(|v| v.usr.clone()).collect();
    let users = state
        .user_repository
        .list_users_by_ids(users)
        .await
        .ok_or_else(construct_error)?;

    // transactions
    let transactions = transactions
        .into_iter()
        .filter_map(|v| {
            // ownership
            let credit_owned = ownerships
                .iter()
                .filter(|&o| o.account == v.credit_account)
                .count()
                > 0;
            let debit_owned = ownerships
                .iter()
                .filter(|&o| o.account == v.debit_account)
                .count()
                > 0;
            let credit_owned_by_self = ownerships
                .iter()
                .filter(|&o| o.usr == user.id)
                .filter(|&o| o.account == v.credit_account)
                .count()
                > 0;
            let debit_owned_by_self = ownerships
                .iter()
                .filter(|&o| o.usr == user.id)
                .filter(|&o| o.account == v.debit_account)
                .count()
                > 0;

            // accounts/users
            let credit = match !credit_owned || credit_owned_by_self {
                true => accounts
                    .iter()
                    .find(|&a| a.id == v.credit_account)
                    .cloned()
                    .map(Either::Left)?,
                false => {
                    let users = ownerships
                        .iter()
                        .filter(|&o| o.account == v.credit_account)
                        .flat_map(|o| users.iter().find(|&u| u.id == o.usr))
                        .cloned()
                        .collect::<Vec<_>>();
                    Either::Right(users)
                }
            };
            let debit = match !debit_owned || debit_owned_by_self {
                true => accounts
                    .iter()
                    .find(|&a| a.id == v.debit_account)
                    .cloned()
                    .map(Either::Left)?,
                false => {
                    let users = ownerships
                        .iter()
                        .filter(|&o| o.account == v.debit_account)
                        .flat_map(|o| users.iter().find(|&u| u.id == o.usr))
                        .cloned()
                        .collect::<Vec<_>>();
                    Either::Right(users)
                }
            };

            // assets
            let credit_asset = assets.iter().find(|&a| a.id == v.credit_asset).cloned()?;
            let debit_asset = assets.iter().find(|&a| a.id == v.debit_asset).cloned()?;

            // amounts
            let credit_amount = v.credit_amount as f64 / 10f64.powi(credit_asset.precision.into());
            let debit_amount = v.debit_amount as f64 / 10f64.powi(debit_asset.precision.into());

            // stamps
            let credit_stamp = v.credit_stamp.and_utc().with_timezone(&tz);
            let debit_stamp = v.debit_stamp.and_utc().with_timezone(&tz);
            let credit_stamp_nice = credit_stamp.format(DATE_TIME_FORMAT_NICE).to_string();
            let debit_stamp_nice = debit_stamp.format(DATE_TIME_FORMAT_NICE).to_string();
            let credit_stamp = credit_stamp.format(DATE_TIME_FORMAT).to_string();
            let debit_stamp = debit_stamp.format(DATE_TIME_FORMAT).to_string();

            let t = TransactionBundle {
                note: v.note,
                credit,
                debit,
                credit_asset,
                debit_asset,
                credit_amount,
                debit_amount,
                credit_stamp,
                debit_stamp,
                credit_stamp_nice,
                debit_stamp_nice,
            };

            Some(t)
        })
        .collect();

    let page = TransactionsList {
        created: query.created,
        transactions: Some(transactions),
    };
    Ok(HtmlTemplate(page))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/transactions", routing::get(handler))
}
