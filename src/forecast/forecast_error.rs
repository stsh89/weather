#[derive(Debug)]
pub enum ForecastError {
    ProviderIsNotValid,
    MissingRequestedDate,
    NoMatchingLocationFound,
    InvalidAddressFormat,
    InvalidCountryCode,
    Unknown,
}
