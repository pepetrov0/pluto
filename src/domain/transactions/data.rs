use chrono::{DateTime, NaiveDateTime};
use chrono_tz::Tz;
use sqlx::FromRow;
use either::Either;

use crate::domain::{accounts::Account, assets::Asset, users::User};

#[derive(Debug, Clone, FromRow)]
pub struct Transaction {
    pub id: String,
    pub note: String,
    pub credit_account: String,
    pub debit_account: String,
    pub credit_asset: String,
    pub debit_asset: String,
    pub credit_stamp: NaiveDateTime,
    pub debit_stamp: NaiveDateTime,
    pub credit_amount: i64,
    pub debit_amount: i64,
    pub credit_settled: bool,
    pub debit_settled: bool,
}

#[derive(Debug)]
pub struct ResolvedTransaction {
    pub note: String,
    pub credit: Either<Account, Vec<User>>,
    pub debit: Either<Account, Vec<User>>,
    pub credit_asset: Asset,
    pub debit_asset: Asset,
    pub credit_amount: f64,
    pub debit_amount: f64,
    pub credit_stamp: DateTime<Tz>,
    pub debit_stamp: DateTime<Tz>,
}
