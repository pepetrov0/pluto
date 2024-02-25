use crate::core::database::{ReadonlyDatabaseRepository, RepositoryError};

use super::{repository::SessionReadonlyRepository, Session};

pub enum SessionQueryError {
    Unknown,
}

pub async fn find<R>(repository: &mut R, id: &str) -> Result<Option<Session>, SessionQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .find_session(id)
        .await
        .map_err(SessionQueryError::from)
}

impl From<RepositoryError> for SessionQueryError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
