use super::Provider;
use super::ProviderError;
use crate::forecast::Weather;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenWeather {
    pub appid: Option<String>,
}

#[derive(Deserialize)]
struct MainPartResponse {
    temp: f64,
}

#[derive(Deserialize)]
struct Response {
    main: MainPartResponse,
}

impl Provider for OpenWeather {
    fn provide(&self, latitude: f64, longitude: f64) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        let client = reqwest::blocking::Client::new();
        let result = client
            .get("https://api.openweathermap.org/data/2.5/weather")
            .query(&[("lat", latitude)])
            .query(&[("lon", longitude)])
            .query(&[("appid", &self.appid)])
            .send();

        match result {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<Response>() {
                    Ok(json) => Ok(Weather {
                        temperature: json.main.temp,
                    }),
                    Err(_error) => Err(ProviderError::Unknown),
                },
                _ => Err(ProviderError::Unknown),
            },
            Err(_error) => Err(ProviderError::Unknown),
        }
    }

    fn is_valid(&self) -> bool {
        self.appid.is_some()
    }
}
