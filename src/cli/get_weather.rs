use super::{CliApp, CliError, DummyProviderConfig, OpenWeatherConfig, ProviderConfig};
use crate::forecast;
use crate::geocode::{search_by_address, Address, Client, GeocodeError, Point};
use crate::providers::{DummyProvider, OpenWeather, Provider};

pub fn run(app: CliApp, address_string: &str) -> Result<(), CliError> {
    let provider: Box<dyn Provider> = match ProviderConfig::try_from(app.config.current_provider) {
        Ok(ProviderConfig::DummyProviderConfig) => dummy_provider(app.config.providers.dummy),
        Ok(ProviderConfig::OpenWeatherConfig) => open_weather(app.config.providers.open_weather),
        Err(_) => return Err(CliError::MissingCurrentProvider),
    };

    let address: Address = match parse_address(address_string) {
        Ok(address) => address,
        Err(error) => return Err(error),
    };

    let point: Point = match geocode_address(app.geocode_client, address) {
        Ok(point) => point,
        Err(GeocodeError::NotFound) => return Err(CliError::AddressNotFound),
        Err(GeocodeError::NothingToGeocode) => return Err(CliError::Unknown),
        Err(GeocodeError::Unknown) => return Err(CliError::Unknown),
        Err(GeocodeError::UnauthorizedClient) => return Err(CliError::UnauthorizedGeocodeClient),
    };

    let request = forecast::Request {
        latitude: point.latitude,
        longitude: point.longitude,
    };

    get_weather(provider, request)
}

fn parse_address(address_string: &str) -> Result<Address, CliError> {
    let split = address_string.split(',');
    let vec: Vec<&str> = split.collect();
    let city: &str;
    let country_alpha_2_code: &str;

    if vec.len() == 2 {
        city = vec[0];
        country_alpha_2_code = vec[1];
    } else {
        return Err(CliError::InvalidAddressFormat);
    }

    if country_alpha_2_code.len() != 2 {
        return Err(CliError::InvalidCountryCode);
    }

    Ok(Address {
        city: Some(city.to_string()),
        country_alpha_2_code: Some(country_alpha_2_code.to_string()),
    })
}

fn geocode_address(client: Box<dyn Client>, address: Address) -> Result<Point, GeocodeError> {
    search_by_address(client, address)
}

fn dummy_provider(config: DummyProviderConfig) -> Box<dyn Provider> {
    Box::new(DummyProvider {
        latitude: Some(config.latitude),
        longitude: Some(config.longitude),
    })
}

fn open_weather(config: OpenWeatherConfig) -> Box<dyn Provider> {
    Box::new(OpenWeather {
        appid: Some(config.appid),
    })
}

fn get_weather(provider: Box<dyn Provider>, request: forecast::Request) -> Result<(), CliError> {
    let result = forecast::show(provider, request);

    match result {
        Ok(weather) => {
            println!("Temperature: {}°C", weather.temperature);
            Ok(())
        }
        Err(_) => Err(CliError::Unknown),
    }
}
