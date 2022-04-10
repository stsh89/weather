use super::{Config, DummyProviderConfig, OpenWeatherConfig, ProviderConfig};
use dialoguer::Input;

pub fn run(config: Config, name: &str) {
    match ProviderConfig::try_from(name.to_string()) {
        Ok(ProviderConfig::DummyProviderConfig) => dummy_provider(config),
        Ok(ProviderConfig::OpenWeatherConfig) => open_weather(config),
        Err(_) => println!("Invalid provider name"),
    }
}

fn dummy_provider(mut config: Config) {
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
}

fn open_weather(mut config: Config) {
    let appid: String = Input::new().with_prompt("appid").interact_text().unwrap();
    config.providers.open_weather = OpenWeatherConfig { appid };
    config.write();
}
