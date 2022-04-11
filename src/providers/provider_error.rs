#[derive(Debug)]
pub enum ProviderError {
    InvalidAddressFormat,
    InvalidConfiguration,
    InvalidCountryCode,
    MissingRequestedDate,
    NoMatchingLocationFound,
    Unauthorized,
    Unknown,
}
