use super::GeoipError;
use super::{Client, ClientResponse};

pub struct OpenWeatherClient {
    appid: String,
}

impl Client for OpenWeatherClient {
    fn search_by_address(&self, q: String) -> Result<ClientResponse, GeoipError> {
        let client = reqwest::blocking::Client::new();
        let result = client
            .get("http://api.openweathermap.org/geo/1.0/direct")
            .query(&["limit", "1"])
            .query(&["q", &q])
            .query(&[("appid", &self.appid)])
            .send();

    match result {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => match response.json::<Vec<ClientResponse>>() {
                Ok(json) => match json.into_iter().nth(0) {
                    Some(response) => Ok(response),
                    None => Err(GeoipError::NotFound),
                },
                Err(_error) => Err(GeoipError::Unknown),
            },
            _ => Err(GeoipError::Unknown),
        },
        Err(_error) => Err(GeoipError::Unknown),
    }
    }
}
