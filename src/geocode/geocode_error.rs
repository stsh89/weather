#[derive(Debug)]
pub enum GeocodeError {
    NotFound,
    NothingToGeocode,
    UnauthorizedClient,
    InvalidAddressFormat,
    InvalidCountryCode,
    Unknown,
}
