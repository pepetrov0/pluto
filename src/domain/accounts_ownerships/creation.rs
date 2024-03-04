use crate::{
    core::database::{RepositoryError, WriteDatabaseRepository},
    domain::{accounts::Account, users::User},
};

use super::{repository::AccountOwnershipWriteRepository, AccountOwnership};

pub enum AccountOwnershipCreationError {
    Unknown,
}

pub async fn create<R>(
    repository: &mut R,
    user: &User,
    account: &Account,
) -> Result<AccountOwnership, AccountOwnershipCreationError>
where
    R: WriteDatabaseRepository,
{
    repository
        .create_account_ownership(&user.id, &account.id)
        .await
        .map_err(AccountOwnershipCreationError::from)
}

impl From<RepositoryError> for AccountOwnershipCreationError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
