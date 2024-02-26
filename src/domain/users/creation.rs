use chrono_tz::Tz;

use crate::{
    core::database::{RepositoryError, WriteDatabaseRepository},
    domain::{accounts::Account, assets::Asset},
    validation,
};

use super::{repository::UserWriteRepository, User, UserQueryError};

pub enum UserCreationError {
    InvalidEmail,
    EmailTaken,
    Unknown,
}

pub async fn create<R>(
    repository: &mut R,
    email: &str,
    password: Option<String>,
    timezone: Tz,
    favorite_asset: &Asset,
    favorite_account: &Account,
) -> Result<User, UserCreationError>
where
    R: WriteDatabaseRepository,
{
    // validate email
    if !validation::is_email(email) {
        return Err(UserCreationError::InvalidEmail);
    }

    // check if email is already taken
    if super::find(repository, email)
        .await
        .map_err(UserCreationError::from)?
        .is_some()
    {
        return Err(UserCreationError::EmailTaken);
    }

    repository
        .create_user(
            email,
            password,
            timezone,
            &favorite_asset.id,
            &favorite_account.id,
        )
        .await
        .map_err(UserCreationError::from)
}

impl From<UserQueryError> for UserCreationError {
    fn from(value: UserQueryError) -> Self {
        match value {
            UserQueryError::Unknown => Self::Unknown,
        }
    }
}

impl From<RepositoryError> for UserCreationError {
    fn from(_: RepositoryError) -> Self {
        Self::Unknown
    }
}
