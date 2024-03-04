use chrono::NaiveDateTime;

use crate::{
    core::database::WriteDatabaseRepository,
    domain::{accounts::Account, assets::Asset},
};

use super::{repository::TransactionWriteRepository, Transaction};

pub enum TransactionCreationError {
    Unknown,
}

pub async fn create<R>(
    repository: &mut R,
    note: &str,
    credit_account: &Account,
    debit_account: &Account,
    credit_asset: &Asset,
    debit_asset: &Asset,
    credit_stamp: NaiveDateTime,
    debit_stamp: NaiveDateTime,
    credit_amount: i64,
    debit_amount: i64,
    credit_settled: bool,
    debit_settled: bool,
) -> Result<Transaction, TransactionCreationError>
where
    R: WriteDatabaseRepository,
{
    repository
        .create_transaction(
            note,
            &credit_account.id,
            &debit_account.id,
            &credit_asset.id,
            &debit_asset.id,
            credit_stamp,
            debit_stamp,
            credit_amount,
            debit_amount,
            credit_settled,
            debit_settled,
        )
        .await
        .ok_or(TransactionCreationError::Unknown)
}
