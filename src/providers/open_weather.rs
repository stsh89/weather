use chrono::NaiveDateTime;

use super::Provider;
use super::ProviderError;
use crate::forecast::Weather;
use crate::geocode::{parse_address, search_by_address, Address, Client, GeocodeError, Point};
use serde::Deserialize;

pub struct OpenWeather {
    pub appid: Option<String>,
    pub geocode_client: Box<dyn Client>,
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
    fn current(&self, address_string: &str) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        let point = match self.geocode(address_string) {
            Ok(point) => point,
            Err(error) => return Err(error),
        };

        let client = reqwest::blocking::Client::new();
        let result = client
            .get("https://api.openweathermap.org/data/2.5/weather")
            .query(&[("lat", point.latitude)])
            .query(&[("lon", point.longitude)])
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

    fn daily(&self, address_string: &str, timestamp: i64) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        let point = match self.geocode(address_string) {
            Ok(point) => point,
            Err(error) => return Err(error),
        };

        let client = reqwest::blocking::Client::new();
        let result = client
            .get("https://api.openweathermap.org/data/2.5/onecall")
            .query(&[("lat", point.latitude)])
            .query(&[("lon", point.longitude)])
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
                reqwest::StatusCode::UNAUTHORIZED => Err(ProviderError::Unauthorized),
                _ => Err(ProviderError::Unknown),
            },
            Err(_error) => Err(ProviderError::Unknown),
        }
    }

    fn is_valid(&self) -> bool {
        self.appid.is_some()
    }
}

impl OpenWeather {
    fn geocode(&self, address_string: &str) -> Result<Point, ProviderError> {
        let address: Address = match parse_address(address_string) {
            Ok(address) => address,
            Err(GeocodeError::InvalidAddressFormat) => {
                return Err(ProviderError::InvalidAddressFormat)
            }
            Err(GeocodeError::InvalidCountryCode) => return Err(ProviderError::InvalidCountryCode),
            _ => return Err(ProviderError::Unknown),
        };

        let result = search_by_address(&*self.geocode_client, &address);

        match result {
            Ok(point) => Ok(point),
            Err(GeocodeError::NotFound) => Err(ProviderError::NoMatchingLocationFound),
            Err(GeocodeError::NothingToGeocode) => Err(ProviderError::NoMatchingLocationFound),
            Err(GeocodeError::Unknown) => Err(ProviderError::Unknown),
            Err(GeocodeError::UnauthorizedClient) => Err(ProviderError::Unauthorized),
            _ => Err(ProviderError::Unknown),
        }
    }
}
