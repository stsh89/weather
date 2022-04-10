use serde::{Deserialize, Serialize};

use crate::forecast::Weather;

use super::Provider;
use super::ProviderError;

#[derive(Deserialize, Serialize, Debug)]
pub struct DummyProvider {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Provider for DummyProvider {
    fn provide(&self, latitude: f64, longitude: f64) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        if latitude == 0.0 && longitude == 0.0 {
            return Err(ProviderError::Unknown);
        }

        if latitude == 0.1 && longitude == 0.1 {
            return Ok(Weather { temperature: 10.22 });
        }

        Ok(Weather { temperature: 10.22 })
    }

    fn is_valid(&self) -> bool {
        matches!((self.latitude, self.longitude), (Some(_), Some(_)))
    }
}

impl Default for DummyProvider {
    fn default() -> Self {
        Self {
            latitude: Some(0.0),
            longitude: Some(0.0),
        }
    }
}
