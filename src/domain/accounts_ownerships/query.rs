use crate::core::database::{ReadonlyDatabaseRepository, RepositoryError};

use super::{repository::AccountOwnershipReadonlyRepository, AccountOwnership};

pub enum AccountOwnershipQueryError {
    Unknown,
}

pub async fn list<R>(
    repository: &mut R,
) -> Result<Vec<AccountOwnership>, AccountOwnershipQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_account_ownerships()
        .await
        .map_err(AccountOwnershipQueryError::from)
}

pub async fn list_by_user_or_account<R>(
    repository: &mut R,
    user_or_account: &str,
) -> Result<Vec<AccountOwnership>, AccountOwnershipQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_account_ownerships_by_user_or_account(user_or_account)
        .await
        .map_err(AccountOwnershipQueryError::from)
}

pub async fn list_by_users_or_accounts<R>(
    repository: &mut R,
    ids: &[String],
) -> Result<Vec<AccountOwnership>, AccountOwnershipQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_account_ownerships_by_users_or_accounts(ids)
        .await
        .map_err(AccountOwnershipQueryError::from)
}

impl From<RepositoryError> for AccountOwnershipQueryError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
