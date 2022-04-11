use super::{
    CliError, Config, DummyProviderConfig, OpenWeatherConfig, ProviderConfig, WeatherapiConfig,
};
use dialoguer::Input;

pub fn run(config: Config, name: &str) -> Result<(), CliError> {
    let result = match ProviderConfig::try_from(name) {
        Ok(ProviderConfig::DummyProvider) => dummy_provider(config),
        Ok(ProviderConfig::OpenWeather) => open_weather(config),
        Ok(ProviderConfig::Weatherapi) => weatherapi(config),
        Err(_) => return Err(CliError::InvalidProviderName),
    };

    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

fn dummy_provider(mut config: Config) -> Result<Config, CliError> {
    let latitude: f64 = Input::new()
        .with_prompt("latitude")
        .interact_text()
        .unwrap();

    let longitude: f64 = Input::new()
        .with_prompt("longitude")
        .interact_text()
        .unwrap();

    config.providers.dummy = DummyProviderConfig {
        latitude,
        longitude,
    };

    config.write();
    Ok(config)
}

fn open_weather(mut config: Config) -> Result<Config, CliError> {
    let appid: String = Input::new().with_prompt("appid").interact_text().unwrap();
    config.providers.open_weather = OpenWeatherConfig { appid };
    config.write();
    Ok(config)
}

fn weatherapi(mut config: Config) -> Result<Config, CliError> {
    let api_key: String = Input::new().with_prompt("api_key").interact_text().unwrap();
    config.providers.weatherapi = WeatherapiConfig { api_key };
    config.write();
    Ok(config)
}
