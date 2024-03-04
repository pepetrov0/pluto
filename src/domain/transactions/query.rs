use crate::{
    core::database::ReadonlyDatabaseRepository,
    domain::transactions::repository::TransactionReadonlyRepository,
};

use super::Transaction;

pub enum TransactionQueryError {
    Unknown,
}

pub async fn count_settled<R>(
    repository: &mut R,
    accounts: &[String],
) -> Result<i64, TransactionQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .count_settled_transactions(accounts)
        .await
        .ok_or(TransactionQueryError::Unknown)
}

pub async fn list_settled<R>(
    repository: &mut R,
    accounts: &[String],
    page_offset: i64,
    page_size: i64,
) -> Result<Vec<Transaction>, TransactionQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_settled_transactions(page_offset, page_size, accounts)
        .await
        .ok_or(TransactionQueryError::Unknown)
}

pub async fn list_unsettled<R>(
    repository: &mut R,
    accounts: &[String],
) -> Result<Vec<Transaction>, TransactionQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_unsettled_transactions(accounts)
        .await
        .ok_or(TransactionQueryError::Unknown)
}
