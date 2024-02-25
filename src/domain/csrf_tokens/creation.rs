

use crate::{database::WriteDatabaseRepository, domain::users::User};

use super::{repository::CsrfTokenRepository, CsrfToken};

pub enum CsrfTokenCreationError {
    Unknown,
}

pub async fn create<R>(
    repository: &mut R,
    user: &User,
    usage: &str,
) -> Result<CsrfToken, CsrfTokenCreationError>
where
    R: WriteDatabaseRepository,
{
    repository
        .create_csrf_token(&user.id, usage)
        .await
        .ok_or(CsrfTokenCreationError::Unknown)
}
