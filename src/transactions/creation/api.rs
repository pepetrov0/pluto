//! Implements transaction creation API

use axum::{extract::State, response::Redirect, Form};
use chrono::{NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use serde::Deserialize;

use crate::{
    accounts::{
        component::{AccountReadonlyRepository, AccountWriteRepository},
        ownership::AccountOwnershipReadonlyRepository,
    },
    assets::component::AssetReadonlyRepository,
    auth::principal::AuthPrincipal,
    database::WriteRepository,
    domain::{self, csrf_tokens},
    transactions::component::TransactionWriteRepository,
    AppState, DATE_TIME_FORMAT,
};

use super::error::TransactionCreationError;

#[derive(Debug, Deserialize)]
pub struct NewTransactionForm {
    pub note: String,
    pub credit_account: String,
    #[serde(default)]
    pub create_credit_account: bool,
    pub debit_account: String,
    #[serde(default)]
    pub create_debit_account: bool,
    pub asset: Option<String>,
    pub credit_asset: Option<String>,
    pub debit_asset: Option<String>,
    pub amount: Option<f64>,
    pub credit_amount: Option<f64>,
    pub debit_amount: Option<f64>,
    pub timestamp: String,
    pub csrf: String,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    State(state): State<AppState>,
    Form(details): Form<NewTransactionForm>,
) -> Result<Redirect, TransactionCreationError> {
    let details = NewTransactionForm {
        note: details.note.trim().to_owned(),
        credit_account: details.credit_account.trim().to_owned(),
        create_credit_account: details.create_credit_account,
        debit_account: details.debit_account.trim().to_owned(),
        create_debit_account: details.create_debit_account,
        asset: details
            .asset
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty()),
        credit_asset: details
            .credit_asset
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty()),
        debit_asset: details
            .debit_asset
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty()),
        amount: details.amount,
        credit_amount: details.credit_amount,
        debit_amount: details.debit_amount,
        timestamp: details.timestamp,
        csrf: details.csrf.trim().to_owned(),
    };

    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or(TransactionCreationError::Unknown)?;

    // check csrf
    if !csrf_tokens::verify(
        &mut repository,
        &details.csrf,
        &user,
        super::CSRF_TOKEN_USAGE,
    )
    .await
    {
        return Err(TransactionCreationError::InvalidCsrf);
    }

    // check for a missing note
    if details.note.is_empty() || details.note.len() > 200 {
        return Err(TransactionCreationError::InvalidNote);
    }

    // timezone and date
    let tz =
        Tz::from_str_insensitive(&user.timezone).map_err(|_| TransactionCreationError::Unknown)?;
    let timestamp = NaiveDateTime::parse_from_str(&details.timestamp, DATE_TIME_FORMAT)
        .map_err(|_| TransactionCreationError::Unknown)?;
    let timestamp = tz
        .from_local_datetime(&timestamp)
        .latest()
        .ok_or(TransactionCreationError::Unknown)?
        .naive_utc()
        .with_second(Utc::now().second())
        .expect("should never happen");

    // account ownerships
    let ownerships = repository
        .list_account_ownerships_by_users_or_accounts(vec![
            details.debit_account.clone(),
            details.credit_account.clone(),
        ])
        .await
        .ok_or(TransactionCreationError::Unknown)?;

    // credit and debit ownership
    let credit_account_owned_by_self = !details.create_credit_account
        && ownerships
            .iter()
            .filter(|&v| v.account == details.credit_account)
            .filter(|&v| v.usr == user.id)
            .count()
            > 0;
    let debit_account_owned_by_self = !details.create_debit_account
        && ownerships
            .iter()
            .filter(|&v| v.account == details.debit_account)
            .filter(|&v| v.usr == user.id)
            .count()
            > 0;
    let credit_account_owned = ownerships
        .iter()
        .filter(|&v| v.usr == details.credit_account || v.account == details.credit_account)
        .count()
        > 0;
    let debit_account_owned = ownerships
        .iter()
        .filter(|&v| v.usr == details.debit_account || v.account == details.debit_account)
        .count()
        > 0;

    // assert account ownerships
    if !credit_account_owned_by_self && !debit_account_owned_by_self {
        return Err(TransactionCreationError::AccountsNotOwned);
    }

    // assets
    let credit_asset = details
        .credit_asset
        .or(details.asset.clone())
        .ok_or(TransactionCreationError::MissingCreditAsset)?;
    let debit_asset = details
        .debit_asset
        .or(details.asset)
        .or(Some(credit_asset.clone()))
        .ok_or(TransactionCreationError::MissingDebitAsset)?;
    let assets = repository
        .list_assets_by_ids(vec![credit_asset.clone(), debit_asset.clone()])
        .await
        .ok_or(TransactionCreationError::Unknown)?;
    let credit_asset = assets
        .iter()
        .find(|&v| v.id == credit_asset)
        .cloned()
        .ok_or(TransactionCreationError::InvalidCreditAsset)?;
    let debit_asset = assets
        .iter()
        .find(|&v| v.id == debit_asset)
        .cloned()
        .ok_or(TransactionCreationError::InvalidDebitAsset)?;

    // amounts
    let credit_amount = details
        .credit_amount
        .or(details.amount)
        .ok_or(TransactionCreationError::MissingCreditAmount)?;
    let debit_amount = details
        .debit_amount
        .or(details.amount)
        .ok_or(TransactionCreationError::MissingDebitAmount)?;
    let credit_amount = (10f64.powi(credit_asset.precision.into()) * credit_amount).round() as i64;
    let debit_amount = (10f64.powi(debit_asset.precision.into()) * debit_amount).round() as i64;

    // check if amounts are within bounds
    if credit_amount <= 0 {
        return Err(TransactionCreationError::InvalidCreditAmount);
    }
    if debit_amount <= 0 {
        return Err(TransactionCreationError::InvalidDebitAmount);
    }

    // users
    let users = domain::users::list_by_ids_or_emails(
        &mut repository,
        &[
            details.credit_account.clone(),
            details.debit_account.clone(),
        ],
    )
    .await
    .ok_or(TransactionCreationError::Unknown)?;

    // accounts
    let accounts = users
        .iter()
        .map(|v| v.favorite_account.clone())
        .chain(
            [
                details.credit_account.clone(),
                details.debit_account.clone(),
            ]
            .into_iter(),
        )
        .collect();
    let accounts = repository
        .list_accounts_by_ids(accounts)
        .await
        .ok_or(TransactionCreationError::Unknown)?;
    let credit_account = match details.create_credit_account {
        true => repository
            .create_account(&details.credit_account)
            .await
            .ok_or(TransactionCreationError::Unknown)?,
        false => {
            let account = users
                .iter()
                .find(|&u| u.id == details.credit_account)
                .map(|v| v.favorite_account.clone())
                .unwrap_or_else(|| details.credit_account.clone());
            accounts
                .iter()
                .find(|&v| v.id == account)
                .cloned()
                .ok_or(TransactionCreationError::MissingDebitAccount)?
        }
    };
    let debit_account = match details.create_debit_account {
        true => repository
            .create_account(&details.debit_account)
            .await
            .ok_or(TransactionCreationError::Unknown)?,
        false => {
            let account = users
                .iter()
                .find(|&u| u.id == details.debit_account)
                .map(|v| v.favorite_account.clone())
                .unwrap_or_else(|| details.debit_account.clone());
            accounts
                .iter()
                .find(|&v| v.id == account)
                .cloned()
                .ok_or(TransactionCreationError::MissingDebitAccount)?
        }
    };

    // assert that accounts are different
    if credit_account.id == debit_account.id {
        return Err(TransactionCreationError::MatchingAccounts);
    }

    repository
        .create_transaction(
            details.note,
            credit_account.id,
            debit_account.id,
            credit_asset.id,
            debit_asset.id,
            timestamp,
            timestamp,
            credit_amount,
            debit_amount,
            credit_account_owned_by_self && !debit_account_owned_by_self || !credit_account_owned,
            debit_account_owned_by_self && !credit_account_owned_by_self || !debit_account_owned,
        )
        .await
        .ok_or(TransactionCreationError::Unknown)?;

    repository
        .commit()
        .await
        .ok_or(TransactionCreationError::Unknown)?;

    Ok(Redirect::to("/transactions?created=true"))
}
