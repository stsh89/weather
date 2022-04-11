use super::{Client, ClientResponse, GeocodeError};

pub struct DummyClient {}

impl Client for DummyClient {
    fn search_by_address(&self, q: String) -> Result<ClientResponse, GeocodeError> {
        if q.is_empty() {
            return Err(GeocodeError::NothingToGeocode);
        }

        if q == "Paris,ZZ" {
            return Err(GeocodeError::NotFound);
        }

        // Actually there is Paris in USA, still we need some value to test unknown error.
        if q == "Paris,US" {
            return Err(GeocodeError::Unknown);
        }

        if q == "Paris,FR" {
            return Ok(ClientResponse {
                lat: 48.8588897,
                lon: 2.3200410217200766,
            });
        }

        Ok(ClientResponse {
            lat: 48.8588897,
            lon: 2.3200410217200766,
        })
    }
}
