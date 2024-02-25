use crate::database::WriteDatabaseRepository;

use super::{repository::AssetWriteRepository, Asset, AssetType};

pub enum AssetCreationError {
    Unknown,
    InvalidLabel,
    InvalidTicker,
    InvalidSymbol,
    InvalidPrecision,
    AlreadyExists,
}

pub async fn create<R>(
    repository: &mut R,
    ticker: &str,
    symbol: Option<&str>,
    label: &str,
    precision: i16,
    atype: AssetType,
) -> Result<Asset, AssetCreationError>
where
    R: WriteDatabaseRepository,
{
    // validate label
    if label.is_empty() || label.len() > 200 {
        return Err(AssetCreationError::InvalidLabel);
    }

    // validate ticker
    if ticker.is_empty() || ticker.len() > 8 {
        return Err(AssetCreationError::InvalidTicker);
    }

    // validate symbol
    if let Some(symbol) = &symbol {
        if symbol.len() > 8 {
            return Err(AssetCreationError::InvalidSymbol);
        }
    }

    // validate precision
    if !(0..=4).contains(&precision) {
        return Err(AssetCreationError::InvalidPrecision);
    }

    if super::find_by_id_or_ticker(repository, ticker)
        .await
        .is_some()
    {
        return Err(AssetCreationError::AlreadyExists);
    }

    let asset = repository
        .create_asset(ticker, symbol, label, precision, atype)
        .await
        .ok_or(AssetCreationError::Unknown)?;
    Ok(asset)
}
