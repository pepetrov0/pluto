//! Implements transaction creation page

use std::collections::HashSet;

use askama::Template;
use axum::extract::{Query, State};
use chrono::Utc;
use chrono_tz::Tz;
use serde::Deserialize;

use crate::{
    accounts::{
        component::{Account, AccountReadonlyRepository},
        ownership::AccountOwnershipReadonlyRepository,
    },
    assets::component::{Asset, AssetReadonlyRepository},
    auth::principal::AuthPrincipal,
    csrf_tokens::{CsrfToken, CsrfTokenRepository},
    templates::HtmlTemplate,
    user::{User, UserReadonlyRepository},
    AppState, DATE_TIME_FORMAT,
};

use super::error::TransactionCreationError;

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct NewTransactionQuery {
    pub new_credit: bool,
    pub new_debit: bool,
    pub multi_asset: bool,
    #[serde(default)]
    pub error: Option<TransactionCreationError>,
}

#[derive(Template, Default)]
#[template(path = "transactions/creation.html")]
pub struct NewTransactionPage {
    pub csrf_token: Option<CsrfToken>,
    pub own_accounts: Option<Vec<Account>>,
    pub other_accounts: Option<Vec<Account>>,
    pub other_users: Option<Vec<User>>,
    pub assets: Option<Vec<Asset>>,
    pub new_credit: bool,
    pub new_debit: bool,
    pub multi_asset: bool,
    pub current_timestamp: String,
    pub error: Option<TransactionCreationError>,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewTransactionQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<NewTransactionPage>, HtmlTemplate<NewTransactionPage>> {
    let construct_error = || HtmlTemplate(NewTransactionPage::default());

    let mut tx = state
        .database
        .begin()
        .await
        .map_err(|_| construct_error())?;

    // create csrf token
    let csrf_token = tx
        .create_csrf_token(&user.id, super::CSRF_TOKEN_USAGE)
        .await;

    // current timestamp
    let tz = Tz::from_str_insensitive(&user.timezone);
    let current_timestamp = tz
        .map(|v| Utc::now().with_timezone(&v))
        .map(|v| v.format(DATE_TIME_FORMAT).to_string())
        .unwrap_or_default();

    // accounts
    let accounts = tx.list_accounts().await.ok_or_else(construct_error)?;

    // accounts ownerships
    let ownerships = tx
        .list_account_ownerships()
        .await
        .ok_or_else(construct_error)?;

    // owned accounts
    let own_accounts: HashSet<_> = ownerships
        .iter()
        .filter(|&v| v.usr == user.id)
        .map(|v| v.account.clone())
        .collect();

    // filter accounts
    let owned_accounts: HashSet<_> = ownerships.iter().map(|v| v.account.clone()).collect();
    let other_accounts = accounts
        .iter()
        .filter(|&v| !owned_accounts.contains(&v.id))
        .cloned()
        .collect();
    let own_accounts = accounts
        .iter()
        .filter(|&v| own_accounts.contains(&v.id))
        .cloned()
        .collect();

    // other users
    let other_users = tx
        .list_users()
        .await
        .ok_or_else(construct_error)?
        .into_iter()
        .filter(|v| v.id != user.id)
        .collect();

    // assets
    let assets = tx.list_assets().await.ok_or_else(construct_error)?;

    tx.commit().await.map_err(|_| construct_error())?;
    let page = NewTransactionPage {
        csrf_token,
        own_accounts: Some(own_accounts),
        other_accounts: Some(other_accounts),
        other_users: Some(other_users),
        assets: Some(assets),
        new_credit: query.new_credit,
        new_debit: query.new_debit,
        multi_asset: query.multi_asset,
        current_timestamp,
        error: query.error,
    };
    Ok(HtmlTemplate(page))
}
