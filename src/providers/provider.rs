use super::ProviderError;
use crate::forecast::Weather;

pub trait Provider {
    fn current(&self, latitude: f64, longitude: f64) -> Result<Weather, ProviderError>;
    fn daily(
        &self,
        latitude: f64,
        longitude: f64,
        timestamp: i64,
    ) -> Result<Weather, ProviderError>;
    fn is_valid(&self) -> bool;
}
