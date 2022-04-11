use super::config::Config;
use super::CliError;

pub fn run(config: &Config, name: &str) -> Result<(), CliError> {
    match name {
        "open_weather" => open_weather_config(config),
        "weatherapi" => weatherapi_config(config),
        _ => return Err(CliError::InvalidProviderName),
    }

    Ok(())
}

fn open_weather_config(config: &Config) {
    let open_weather = &config.providers.open_weather;
    println!("Appid: {:?}", open_weather.appid);
}

fn weatherapi_config(config: &Config) {
    let weatherapi = &config.providers.weatherapi;
    println!("Api key: {:?}", weatherapi.api_key);
}
