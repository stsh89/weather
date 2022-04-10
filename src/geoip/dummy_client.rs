use super::GeoipError;
use super::{Client, ClientResponse};

pub struct DummyClient {}

impl Client for DummyClient {
    fn search_by_address(&self, q: String) -> Result<ClientResponse, GeoipError> {
        if q == "" {
            return Err(GeoipError::NothingToGeocode);
        }

        if q == "Paris,ZZ" {
            return Err(GeoipError::NotFound);
        }

        // Actually there is Paris in USA, still we need some value to test unknown error.
        if q == "Paris,US" {
            return Err(GeoipError::Unknown);
        }

        if q == "Paris,FR" {
            return Ok(ClientResponse {
                lat: 48.8588897,
                lon: 2.3200410217200766,
            });
        }

        return Ok(ClientResponse {
            lat: 48.8588897,
            lon: 2.3200410217200766,
        });
    }
}
