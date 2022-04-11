use super::{CliError, Config, OpenWeatherConfig, ProviderConfig, WeatherapiConfig};
use dialoguer::Input;

pub fn run(config: Config, name: &str) -> Result<(), CliError> {
    let result = match ProviderConfig::try_from(name) {
        Ok(ProviderConfig::OpenWeather) => open_weather(config),
        Ok(ProviderConfig::Weatherapi) => weatherapi(config),
        Err(_) => return Err(CliError::InvalidProviderName),
    };

    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
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
