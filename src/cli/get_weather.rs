use chrono::NaiveDateTime;

use super::{CliApp, CliError, OpenWeatherConfig, ProviderConfig, WeatherapiConfig};
use crate::forecast::{current, daily, ForecastError, Weather};
use crate::geocode::OpenWeatherClient;
use crate::providers::{OpenWeather, Provider, Weatherapi};

pub fn run(app: &CliApp, address_string: &str, date: &Option<String>) -> Result<(), CliError> {
    let current_provider: &str = &app.config.current_provider;
    let provider: Box<dyn Provider> = match ProviderConfig::try_from(current_provider) {
        Ok(ProviderConfig::OpenWeather) => open_weather(&app.config.providers.open_weather),
        Ok(ProviderConfig::Weatherapi) => weatherapi(&app.config.providers.weatherapi),
        Err(_) => return Err(CliError::MissingCurrentProvider),
    };

    get_weather(&*provider, address_string, date)
}

fn open_weather(config: &OpenWeatherConfig) -> Box<dyn Provider> {
    Box::new(OpenWeather {
        appid: Some(config.appid.clone()),
        geocode_client: Box::new(OpenWeatherClient {
            appid: config.appid.clone(),
        }),
    })
}

fn weatherapi(config: &WeatherapiConfig) -> Box<dyn Provider> {
    Box::new(Weatherapi {
        api_key: Some(config.api_key.clone()),
    })
}

fn get_weather(
    provider: &dyn Provider,
    address_string: &str,
    date: &Option<String>,
) -> Result<(), CliError> {
    match date {
        Some(date_string) => date_weather(provider, address_string, date_string),
        None => current_weather(provider, address_string),
    }
}

fn date_weather(
    provider: &dyn Provider,
    address_string: &str,
    date_string: &str,
) -> Result<(), CliError> {
    let timestamp = match parse_date(date_string) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let result = daily(provider, address_string, timestamp);
    process_forecast_result(result)
}

fn parse_date(date_string: &str) -> Result<i64, CliError> {
    let parse_result =
        NaiveDateTime::parse_from_str(&format!("{} 00:00:00", date_string), "%Y-%m-%d %H:%M:%S");

    match parse_result {
        Ok(date) => Ok(date.timestamp()),
        Err(_) => Err(CliError::InvalidDateFormat),
    }
}

fn current_weather(provider: &dyn Provider, address_string: &str) -> Result<(), CliError> {
    let result = current(provider, address_string);
    process_forecast_result(result)
}

fn process_forecast_result(result: Result<Weather, ForecastError>) -> Result<(), CliError> {
    match result {
        Ok(weather) => {
            println!("Temperature: {}°C", weather.temperature);
            Ok(())
        }
        Err(ForecastError::UnauthorizedProvider) => Err(CliError::ProviderIsNotConfigured),
        Err(ForecastError::NoMatchingLocationFound) => Err(CliError::AddressNotFound),
        Err(ForecastError::InvalidAddressFormat) => Err(CliError::InvalidAddressFormat),
        Err(ForecastError::InvalidCountryCode) => Err(CliError::InvalidCountryCode),
        Err(ForecastError::Unknown) => Err(CliError::Unknown),
        Err(ForecastError::ProviderIsNotValid) => Err(CliError::ProviderIsNotConfigured),
        Err(ForecastError::MissingRequestedDate) => Err(CliError::MissingRequestedDate),
    }
}
