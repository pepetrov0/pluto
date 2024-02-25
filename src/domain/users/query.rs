use crate::core::database::ReadonlyDatabaseRepository;

use super::{repository::UserReadonlyRepository, User, UserWithPassword};

pub async fn find<R>(repository: &mut R, id_or_email: &str) -> Option<User>
where
    R: UserReadonlyRepository,
{
    repository.find_user(id_or_email).await
}

pub async fn find_with_password<R>(
    repository: &mut R,
    id_or_email: &str,
) -> Option<UserWithPassword>
where
    R: ReadonlyDatabaseRepository,
{
    repository.find_user_with_password(id_or_email).await
}

pub async fn list<R>(repository: &mut R) -> Option<Vec<User>>
where
    R: ReadonlyDatabaseRepository,
{
    repository.list_users().await
}

pub async fn list_by_ids_or_emails<R>(
    repository: &mut R,
    ids_or_emails: &[String],
) -> Option<Vec<User>>
where
    R: ReadonlyDatabaseRepository,
{
    repository.list_users_by_ids_or_emails(ids_or_emails).await
}
