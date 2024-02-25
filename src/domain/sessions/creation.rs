use crate::{
    core::database::{RepositoryError, WriteDatabaseRepository},
    domain::users::User,
};

use super::{repository::SessionWriteRepository, Session};

pub enum SessionCreationError {
    Unknown,
}

pub async fn create<R>(repository: &mut R, user: &User) -> Result<Session, SessionCreationError>
where
    R: WriteDatabaseRepository,
{
    repository
        .create_session(&user.id)
        .await
        .map_err(SessionCreationError::from)
}

impl From<RepositoryError> for SessionCreationError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
