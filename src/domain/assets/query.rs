use crate::database::ReadonlyDatabaseRepository;

use super::{repository::AssetReadonlyRepository, Asset};

pub async fn find_by_id_or_ticker<R>(repository: &mut R, id_or_ticker: &str) -> Option<Asset>
where
    R: ReadonlyDatabaseRepository,
{
    repository.find_asset(id_or_ticker).await
}

pub async fn list<R>(repository: &mut R) -> Option<Vec<Asset>>
where
    R: ReadonlyDatabaseRepository,
{
    repository.list_assets().await
}

pub async fn list_by_ids_or_tickers<R>(
    repository: &mut R,
    ids_or_tickers: &[String],
) -> Option<Vec<Asset>>
where
    R: ReadonlyDatabaseRepository,
{
    repository
        .list_assets_by_ids_or_tickers(ids_or_tickers)
        .await
}
