use super::provider::{Error, Provider};
use crate::forecast::Weather;

pub struct DummyProvider;

impl Provider for DummyProvider {
    fn provide(&self, latitude: f64, longitude: f64) -> Result<Weather, Error> {
        if latitude == 0.0 && longitude == 0.0 {
            return Err(Error::Unknown);
        }

        if latitude == 0.1 && longitude == 0.1 {
            return Ok(Weather { temperature: 10.22 });
        }

        Ok(Weather { temperature: 10.22 })
    }
}
