use super::ProviderError;
use crate::forecast::Weather;

pub trait Provider {
    fn current(&self, address_string: &str) -> Result<Weather, ProviderError>;
    fn daily(&self, address_string: &str, timestamp: i64) -> Result<Weather, ProviderError>;
    fn is_valid(&self) -> bool;
}
