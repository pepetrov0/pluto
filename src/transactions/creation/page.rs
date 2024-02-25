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
    auth::principal::AuthPrincipal,
    core::database::WriteRepository,
    core::web::templates::HtmlTemplate,
    domain::{
        self,
        assets::Asset,
        csrf_tokens::{self, CsrfToken},
        users::User,
    },
    AppState, DATE_TIME_FORMAT,
};

use super::error::TransactionCreationError;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct NewTransactionPresetQuery {}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct NewTransactionQuery {
    pub error: Option<TransactionCreationError>,
    #[serde(rename = "preset[new-credit]")]
    pub new_credit: bool,
    #[serde(rename = "preset[new-debit]")]
    pub new_debit: bool,
    #[serde(rename = "preset[multi-asset]")]
    pub multi_asset: bool,
}

#[derive(Debug)]
pub struct NewTransactionPreset {
    pub credit_account: String,
    pub debit_account: String,
    pub asset: String,
    pub credit_asset: String,
    pub debit_asset: String,
    pub new_credit: bool,
    pub new_debit: bool,
    pub multi_asset: bool,
    pub timestamp: String,
}

#[derive(Template)]
#[template(path = "transactions/creation.html")]
pub struct NewTransactionPage {
    pub csrf_token: Option<CsrfToken>,
    pub own_accounts: Option<Vec<Account>>,
    pub other_accounts: Option<Vec<Account>>,
    pub other_users: Option<Vec<User>>,
    pub assets: Option<Vec<Asset>>,
    pub preset: Option<NewTransactionPreset>,
    pub error: Option<TransactionCreationError>,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    Query(query): Query<NewTransactionQuery>,
    State(state): State<AppState>,
) -> Result<HtmlTemplate<NewTransactionPage>, HtmlTemplate<NewTransactionPage>> {
    let construct_error = || {
        HtmlTemplate(NewTransactionPage {
            csrf_token: None,
            own_accounts: None,
            other_accounts: None,
            other_users: None,
            assets: None,
            preset: None,
            error: None,
        })
    };
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or_else(construct_error)?;

    // current timestamp
    let tz = Tz::from_str_insensitive(&user.timezone);
    let current_timestamp = tz
        .map(|v| Utc::now().with_timezone(&v))
        .map(|v| v.format(DATE_TIME_FORMAT).to_string())
        .unwrap_or_default();

    // create csrf token
    let csrf_token = csrf_tokens::create(&mut repository, &user, super::CSRF_TOKEN_USAGE)
        .await
        .ok();

    // accounts
    let accounts = repository
        .list_accounts()
        .await
        .ok_or_else(construct_error)?;

    // accounts ownerships
    let ownerships = repository
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
    let other_users = domain::users::list(&mut repository)
        .await
        .map_err(|_| construct_error())?
        .into_iter()
        .filter(|v| v.id != user.id)
        .collect();

    // assets
    let assets = domain::assets::list(&mut repository)
        .await
        .ok_or_else(construct_error)?;

    // preset
    let preset = NewTransactionPreset {
        credit_account: user.favorite_account.clone(),
        debit_account: user.favorite_account.clone(),
        asset: user.favorite_asset.clone(),
        credit_asset: user.favorite_asset.clone(),
        debit_asset: user.favorite_asset.clone(),
        new_credit: query.new_credit && !query.new_debit,
        new_debit: query.new_debit && !query.new_credit,
        multi_asset: query.multi_asset,
        timestamp: current_timestamp,
    };

    repository.commit().await.ok_or_else(construct_error)?;
    let page = NewTransactionPage {
        csrf_token,
        own_accounts: Some(own_accounts),
        other_accounts: Some(other_accounts),
        other_users: Some(other_users),
        assets: Some(assets),
        preset: Some(preset),
        error: query.error,
    };
    Ok(HtmlTemplate(page))
}
