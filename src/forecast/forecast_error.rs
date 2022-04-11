#[derive(Debug)]
pub enum ForecastError {
    UnauthorizedProvider,
    ProviderIsNotValid,
    MissingRequestedDate,
    NoMatchingLocationFound,
    InvalidAddressFormat,
    InvalidCountryCode,
    Unknown,
}
