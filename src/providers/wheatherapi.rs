use chrono::NaiveDateTime;
use serde::Deserialize;

use super::{Provider, ProviderError};
use crate::forecast::Weather;

pub struct Weatherapi {
    pub api_key: Option<String>,
}

#[derive(Deserialize)]
struct CurrentResponse {
    current: CurrentPartResponse,
}

#[derive(Deserialize)]
struct CurrentPartResponse {
    temp_c: f64,
}

#[derive(Deserialize)]
struct DailyResponse {
    forecast: ForecastPart,
}

#[derive(Deserialize)]
struct ForecastPart {
    forecastday: Vec<ForecastDayPart>,
}

#[derive(Deserialize)]
struct ForecastDayPart {
    date_epoch: i64,
    day: DayPart,
}

#[derive(Deserialize)]
struct DayPart {
    avgtemp_c: f64,
}

impl Provider for Weatherapi {
    fn current(&self, address_string: &str) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        let client = reqwest::blocking::Client::new();
        let result = client
            .get("http://api.weatherapi.com/v1/current.json")
            .query(&[("q", address_string)])
            .query(&[("key", &self.api_key)])
            .send();

        match result {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<CurrentResponse>() {
                    Ok(json) => Ok(Weather {
                        temperature: json.current.temp_c,
                    }),
                    Err(_error) => Err(ProviderError::Unknown),
                },
                reqwest::StatusCode::UNAUTHORIZED => Err(ProviderError::Unauthorized),
                reqwest::StatusCode::BAD_REQUEST => Err(ProviderError::NoMatchingLocationFound),
                _ => Err(ProviderError::Unknown),
            },
            Err(_error) => Err(ProviderError::Unknown),
        }
    }

    fn daily(&self, address_string: &str, timestamp: i64) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        let client = reqwest::blocking::Client::new();
        let result = client
            .get("http://api.weatherapi.com/v1/forecast.json")
            .query(&[("days", "10")])
            .query(&[("q", address_string)])
            .query(&[("key", &self.api_key)])
            .send();

        let date = NaiveDateTime::from_timestamp(timestamp, 0)
            .format("%Y-%m-%d")
            .to_string();

        match result {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<DailyResponse>() {
                    Ok(json) => {
                        let found = json.forecast.forecastday.into_iter().find(|value| {
                            let ndt = NaiveDateTime::from_timestamp(value.date_epoch, 0)
                                .format("%Y-%m-%d")
                                .to_string();
                            date == ndt
                        });
                        match found {
                            Some(list_item) => Ok(Weather {
                                temperature: list_item.day.avgtemp_c,
                            }),
                            None => Err(ProviderError::MissingRequestedDate),
                        }
                    }
                    Err(_) => Err(ProviderError::Unknown),
                },
                reqwest::StatusCode::UNAUTHORIZED => Err(ProviderError::Unauthorized),
                reqwest::StatusCode::BAD_REQUEST => Err(ProviderError::NoMatchingLocationFound),
                _ => Err(ProviderError::Unknown),
            },
            Err(_error) => Err(ProviderError::Unknown),
        }
    }

    fn is_valid(&self) -> bool {
        self.api_key.is_some()
    }
}
