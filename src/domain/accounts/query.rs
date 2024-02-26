use crate::core::database::{ReadonlyDatabaseRepository, RepositoryError};

use super::{repository::AccountReadonlyRepository, Account};

pub enum AccountQueryError {
    Unknown,
}

pub async fn list<R>(repository: &mut R) -> Result<Vec<Account>, AccountQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_accounts()
        .await
        .map_err(AccountQueryError::from)
}

pub async fn list_by_ids<R>(
    repository: &mut R,
    ids: &[String],
) -> Result<Vec<Account>, AccountQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_accounts_by_ids(ids)
        .await
        .map_err(AccountQueryError::from)
}

impl From<RepositoryError> for AccountQueryError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
