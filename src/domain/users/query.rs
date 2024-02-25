use crate::core::database::{ReadonlyDatabaseRepository, RepositoryError};

use super::{repository::UserReadonlyRepository, User, UserWithPassword};

pub enum UserQueryError {
    Unknown,
}

pub async fn find<R>(
    repository: &mut R,
    id_or_email: &str,
) -> Result<Option<User>, UserQueryError>
where
    R: UserReadonlyRepository,
{
    repository
        .find_user(id_or_email)
        .await
        .map_err(UserQueryError::from)
}

pub async fn find_with_password<R>(
    repository: &mut R,
    id_or_email: &str,
) -> Result<Option<UserWithPassword>, UserQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .find_user_with_password(id_or_email)
        .await
        .map_err(UserQueryError::from)
}

pub async fn list<R>(repository: &mut R) -> Result<Vec<User>, UserQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository.list_users().await.map_err(UserQueryError::from)
}

pub async fn list_by_ids_or_emails<R>(
    repository: &mut R,
    ids_or_emails: &[String],
) -> Result<Vec<User>, UserQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_users_by_ids_or_emails(ids_or_emails)
        .await
        .map_err(UserQueryError::from)
}

impl From<RepositoryError> for UserQueryError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
