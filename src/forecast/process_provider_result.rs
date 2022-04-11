use super::{ForecastError, Weather};
use crate::providers::ProviderError;

pub fn run(result: Result<Weather, ProviderError>) -> Result<Weather, ForecastError> {
    match result {
        Ok(weather) => Ok(weather),
        Err(ProviderError::Unauthorized) => Err(ForecastError::UnauthorizedProvider),
        Err(ProviderError::Unknown) => Err(ForecastError::Unknown),
        Err(ProviderError::InvalidConfiguration) => Err(ForecastError::ProviderIsNotValid),
        Err(ProviderError::MissingRequestedDate) => Err(ForecastError::MissingRequestedDate),
        Err(ProviderError::NoMatchingLocationFound) => Err(ForecastError::NoMatchingLocationFound),
        Err(ProviderError::InvalidAddressFormat) => Err(ForecastError::InvalidAddressFormat),
        Err(ProviderError::InvalidCountryCode) => Err(ForecastError::InvalidCountryCode),
    }
}
