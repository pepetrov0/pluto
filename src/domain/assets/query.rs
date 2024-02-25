use crate::core::database::{ReadonlyDatabaseRepository, RepositoryError};

use super::{repository::AssetReadonlyRepository, Asset};

pub enum AssetQueryError {
    Unknown,
}

pub async fn find_by_id_or_ticker<R>(
    repository: &mut R,
    id_or_ticker: &str,
) -> Result<Option<Asset>, AssetQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .find_asset(id_or_ticker)
        .await
        .map_err(AssetQueryError::from)
}

pub async fn list<R>(repository: &mut R) -> Result<Vec<Asset>, AssetQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_assets()
        .await
        .map_err(AssetQueryError::from)
}

pub async fn list_by_ids_or_tickers<R>(
    repository: &mut R,
    ids_or_tickers: &[String],
) -> Result<Vec<Asset>, AssetQueryError>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_assets_by_ids_or_tickers(ids_or_tickers)
        .await
        .map_err(AssetQueryError::from)
}

impl From<RepositoryError> for AssetQueryError {
    fn from(value: RepositoryError) -> Self {
        Self::Unknown
    }
}
