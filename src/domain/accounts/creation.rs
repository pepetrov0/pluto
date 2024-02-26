use crate::core::database::{RepositoryError, WriteDatabaseRepository};

use super::{repository::AccountWriteRepository, Account};

pub enum AccountCreationError {
    InvalidName,
    Unknown,
}

pub async fn create<R>(repository: &mut R, name: &str) -> Result<Account, AccountCreationError>
where
    R: WriteDatabaseRepository,
{
    // check for a missing name
    if name.is_empty() || name.len() > 200 {
        return Err(AccountCreationError::InvalidName);
    }

    repository
        .create_account(name)
        .await
        .map_err(AccountCreationError::from)
}

impl From<RepositoryError> for AccountCreationError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
