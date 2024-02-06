//! Implements transaction creation page

use std::collections::HashSet;

use askama::Template;
use axum::extract::{Query, State};
use serde::Deserialize;

use crate::{
    accounts::component::Account, assets::component::Asset, auth::principal::AuthPrincipal,
    csrf_tokens::CsrfToken, templates::HtmlTemplate, user::User, AppState,
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
    pub error: Option<TransactionCreationError>,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewTransactionQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<NewTransactionPage>, HtmlTemplate<NewTransactionPage>> {
    // create csrf token
    let csrf_token = state
        .csrf_token_repository
        .create_csrf_token(user.id.clone(), super::CSRF_TOKEN_USAGE)
        .await;

    // accounts
    let accounts = state
        .account_repository
        .list_accounts()
        .await
        .ok_or_else(|| HtmlTemplate(NewTransactionPage::default()))?;

    // accounts ownerships
    let ownerships = state
        .account_ownership_repository
        .list_account_ownerships()
        .await
        .ok_or_else(|| HtmlTemplate(NewTransactionPage::default()))?;

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
    let other_users = state
        .user_repository
        .list_users()
        .await
        .ok_or_else(|| HtmlTemplate(NewTransactionPage::default()))?
        .into_iter()
        .filter(|v| v.id != user.id)
        .collect();

    // assets
    let assets = state
        .asset_repository
        .list_assets()
        .await
        .ok_or_else(|| HtmlTemplate(NewTransactionPage::default()))?;

    let page = NewTransactionPage {
        csrf_token,
        own_accounts: Some(own_accounts),
        other_accounts: Some(other_accounts),
        other_users: Some(other_users),
        assets: Some(assets),
        new_credit: query.new_credit,
        new_debit: query.new_debit,
        multi_asset: query.multi_asset,
        error: query.error,
    };
    Ok(HtmlTemplate(page))
}
