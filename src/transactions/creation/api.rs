//! Implements transaction creation API

use axum::{extract::State, response::Redirect, Form};
use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use serde::Deserialize;

use crate::{accounts::{component::AccountRepository, ownership::AccountOwnershipRepository}, assets::component::AssetRepository, auth::principal::AuthPrincipal, transactions::component::TransactionRepository, AppState, DATE_TIME_FORMAT};

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
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    State(mut state): State<AppState>,
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
    };

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
        .naive_utc();

    // account ownerships
    let ownerships = state
        .database
        .list_account_ownerships_by_accounts(vec![
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
        .filter(|&v| v.account == details.credit_account)
        .count()
        > 0;
    let debit_account_owned = ownerships
        .iter()
        .filter(|&v| v.account == details.debit_account)
        .count()
        > 0;

    // assert account ownerships
    if !credit_account_owned_by_self && !debit_account_owned_by_self {
        return Err(TransactionCreationError::AccountsNotOwned);
    }

    // accounts
    let accounts = state
        .database
        .list_accounts_by_ids(vec![
            details.debit_account.clone(),
            details.credit_account.clone(),
        ])
        .await
        .ok_or(TransactionCreationError::Unknown)?;

    // credit and debit accounts
    let credit_account = match details.create_credit_account {
        true => None,
        false => accounts
            .iter()
            .find(|&v| v.id == details.credit_account)
            .cloned(),
    };
    let debit_account = match details.create_debit_account {
        true => None,
        false => accounts
            .iter()
            .find(|&v| v.id == details.debit_account)
            .cloned(),
    };

    // assert that accounts are different
    if credit_account.as_ref().map(|v| &v.id) == debit_account.as_ref().map(|v| &v.id) {
        return Err(TransactionCreationError::MatchingAccounts);
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
    let assets = state
        .database
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

    // account creation
    let credit_account = match credit_account {
        Some(account) => account,
        None if details.create_credit_account => state
            .database
            .create_account(details.credit_account)
            .await
            .ok_or(TransactionCreationError::Unknown)?,
        None => return Err(TransactionCreationError::MissingCreditAccount),
    };
    let debit_account = match debit_account {
        Some(account) => account,
        None if details.create_debit_account => state
            .database
            .create_account(details.debit_account)
            .await
            .ok_or(TransactionCreationError::Unknown)?,
        None => return Err(TransactionCreationError::MissingDebitAccount),
    };

    state
        .database
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
            credit_account_owned_by_self || !credit_account_owned,
            debit_account_owned_by_self || !debit_account_owned,
        )
        .await
        .map(|_| Redirect::to("/transactions?created=true"))
        .ok_or(TransactionCreationError::Unknown)
}
