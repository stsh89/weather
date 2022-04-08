use super::provider::{Error, Provider};
use crate::forecast::Weather;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DummyProvider {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Provider for DummyProvider {
    fn provide(&self, latitude: f64, longitude: f64) -> Result<Weather, Error> {
        if !self.valid() {
            return Err(Error::InvalidConfiguration);
        }

        if latitude == 0.0 && longitude == 0.0 {
            return Err(Error::Unknown);
        }

        if latitude == 0.1 && longitude == 0.1 {
            return Ok(Weather { temperature: 10.22 });
        }

        Ok(Weather { temperature: 10.22 })
    }

    fn valid(&self) -> bool {
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
