use sqlx::Postgres;

use crate::{database::WriteDatabaseRepository, domain::users::User};

use super::repository::CsrfTokenRepository;

pub async fn verify<R>(repository: &mut R, id: &str, user: &User, usage: &str) -> bool
where
    R: WriteDatabaseRepository<Postgres>,
{
    repository
        .consume_csrf_token(id)
        .await
        .filter(|v| v.usr == user.id)
        .filter(|v| v.usage == usage)
        .is_some()
}
