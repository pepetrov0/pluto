use chrono_tz::Tz;

use crate::{accounts::component::Account, assets::component::Asset, validation};

use super::{
    repository::{UserReadonlyRepository, UserWriteRepository},
    User,
};

pub enum AccountCreationError {
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
) -> Result<User, AccountCreationError>
where
    R: UserReadonlyRepository + UserWriteRepository,
{
    // validate email
    if !validation::is_email(email) {
        return Err(AccountCreationError::InvalidEmail);
    }

    // check if email is already taken
    if super::find(repository, email).await.is_some() {
        return Err(AccountCreationError::EmailTaken);
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
        .ok_or(AccountCreationError::Unknown)
}
