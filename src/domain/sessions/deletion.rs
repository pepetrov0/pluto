use crate::core::database::{RepositoryError, WriteDatabaseRepository};

use super::{repository::SessionWriteRepository, Session};

pub enum SessionDeletionError {
    Unknown,
}

pub async fn delete<R>(repository: &mut R, session: &Session) -> Result<(), SessionDeletionError>
where
    R: WriteDatabaseRepository,
{
    repository
        .delete_session(&session.id)
        .await
        .map_err(SessionDeletionError::from)
}

impl From<RepositoryError> for SessionDeletionError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
