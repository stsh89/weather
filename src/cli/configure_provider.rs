use super::{CliError, Config, DummyProviderConfig, OpenWeatherConfig, ProviderConfig};
use dialoguer::Input;

pub fn run(config: Config, name: &str) -> Result<(), CliError> {
    match ProviderConfig::try_from(name.to_string()) {
        Ok(ProviderConfig::DummyProviderConfig) => dummy_provider(config),
        Ok(ProviderConfig::OpenWeatherConfig) => open_weather(config),
        Err(_) => Err(CliError::InvalidProviderName),
    }
}

fn dummy_provider(mut config: Config) -> Result<(), CliError> {
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
    Ok(())
}

fn open_weather(mut config: Config) -> Result<(), CliError> {
    let appid: String = Input::new().with_prompt("appid").interact_text().unwrap();
    config.providers.open_weather = OpenWeatherConfig { appid };
    config.write();
    Ok(())
}
