use super::ProviderError;
use crate::forecast::Weather;

pub trait Provider {
    fn provide(&self, latitude: f64, longitude: f64) -> Result<Weather, ProviderError>;
    fn is_valid(&self) -> bool;
}
