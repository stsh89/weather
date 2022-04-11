pub enum CliError {
    AddressNotFound,
    InvalidAddressFormat,
    InvalidCountryCode,
    InvalidDateFormat,
    InvalidProviderName,
    MissingCurrentProvider,
    MissingRequestedDate,
    UnauthorizedGeocodeClient,
    Unknown,
}
