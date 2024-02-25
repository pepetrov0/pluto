use crate::{
    core::database::{RepositoryError, WriteDatabaseRepository},
    domain::users::User,
};

use super::repository::CsrfTokenRepository;

pub enum CsrfTokenVerificationError {
    Unknown,
}

pub async fn verify<R>(
    repository: &mut R,
    id: &str,
    user: &User,
    usage: &str,
) -> Result<bool, CsrfTokenVerificationError>
where
    R: WriteDatabaseRepository,
{
    let result = repository
        .consume_csrf_token(id)
        .await
        .map_err(CsrfTokenVerificationError::from)?
        .filter(|v| v.usr == user.id)
        .filter(|v| v.usage == usage)
        .is_some();
    Ok(result)
}

impl From<RepositoryError> for CsrfTokenVerificationError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
