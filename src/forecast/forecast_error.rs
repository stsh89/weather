#[derive(Debug)]
pub enum ForecastError {
    ProviderIsNotValid,
    MissingRequestedDate,
    Unknown,
}
