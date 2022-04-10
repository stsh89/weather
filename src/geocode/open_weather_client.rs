use super::GeocodeError;
use super::{Client, ClientResponse};

pub struct OpenWeatherClient {
    pub appid: String,
}

impl Client for OpenWeatherClient {
    fn search_by_address(&self, q: String) -> Result<ClientResponse, GeocodeError> {
        let client = reqwest::blocking::Client::new();
        let result = client
            .get("http://api.openweathermap.org/geo/1.0/direct")
            .query(&[("limit", 1)])
            .query(&[("q", q)])
            .query(&[("appid", &self.appid)])
            .send();

        match result {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<Vec<ClientResponse>>() {
                    Ok(json) => match json.into_iter().next() {
                        Some(response) => Ok(response),
                        None => Err(GeocodeError::NotFound),
                    },
                    Err(_error) => Err(GeocodeError::Unknown),
                },
                reqwest::StatusCode::UNAUTHORIZED => Err(GeocodeError::UnauthorizedClient),
                _ => Err(GeocodeError::Unknown),
            },
            Err(_error) => Err(GeocodeError::Unknown),
        }
    }
}
