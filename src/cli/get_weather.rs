use super::{Config, DummyProviderConfig, OpenWeatherConfig, ProviderConfig};
use crate::forecast;
use crate::providers::{DummyProvider, OpenWeather, Provider};

pub fn run(config: Config, _address: &str) {
    match ProviderConfig::try_from(config.current_provider) {
        Ok(ProviderConfig::DummyProviderConfig) => dummy_provider(config.providers.dummy),
        Ok(ProviderConfig::OpenWeatherConfig) => open_weather(config.providers.open_weather),
        Err(_) => println!("Set correct provider"),
    }
}

fn dummy_provider(config: DummyProviderConfig) {
    let provider = DummyProvider {
        latitude: Some(config.latitude),
        longitude: Some(config.longitude),
    };
    get_weather(Box::new(provider));
}

fn open_weather(config: OpenWeatherConfig) {
    let provider = OpenWeather {
        appid: Some(config.appid),
    };
    get_weather(Box::new(provider));
}

fn get_weather(provider: Box<dyn Provider>) {
    let request = forecast::Request {
        latitude: 51.5073219,
        longitude: -0.1276474,
    };

    let result = forecast::show(provider, request);

    match result {
        Ok(weather) => println!("Temperature: {}", weather.temperature),
        Err(error) => println!("Error: {:?}", error),
    }
}
