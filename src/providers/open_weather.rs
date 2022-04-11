use chrono::NaiveDateTime;

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
struct TempPartResponse {
    day: f64,
}

#[derive(Deserialize)]
struct CurrentResponse {
    main: MainPartResponse,
}

#[derive(Deserialize)]
struct DailyResponse {
    daily: Vec<DailyListResponse>,
}

#[derive(Deserialize)]
struct DailyListResponse {
    dt: i64,
    temp: TempPartResponse,
}

impl Provider for OpenWeather {
    fn current(&self, latitude: f64, longitude: f64) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        let client = reqwest::blocking::Client::new();
        let result = client
            .get("https://api.openweathermap.org/data/2.5/weather")
            .query(&[("lat", latitude)])
            .query(&[("lon", longitude)])
            .query(&[("units", "metric")])
            .query(&[("appid", &self.appid)])
            .send();

        match result {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<CurrentResponse>() {
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

    fn daily(
        &self,
        latitude: f64,
        longitude: f64,
        timestamp: i64,
    ) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        let client = reqwest::blocking::Client::new();
        let result = client
            .get("https://api.openweathermap.org/data/2.5/onecall")
            .query(&[("lat", latitude)])
            .query(&[("lon", longitude)])
            .query(&[("units", "metric")])
            .query(&[("appid", &self.appid)])
            .send();

        let date = NaiveDateTime::from_timestamp(timestamp, 0)
            .format("%Y-%m-%d")
            .to_string();

        match result {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<DailyResponse>() {
                    Ok(json) => {
                        let found = json.daily.into_iter().find(|list_item| {
                            let ndt = NaiveDateTime::from_timestamp(list_item.dt, 0)
                                .format("%Y-%m-%d")
                                .to_string();
                            date == ndt
                        });
                        match found {
                            Some(list_item) => Ok(Weather {
                                temperature: list_item.temp.day,
                            }),
                            None => Err(ProviderError::MissingRequestedDate),
                        }
                    }
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
