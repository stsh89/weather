use super::CliError;
use super::ProviderConfig;

pub fn run() -> Result<(), CliError> {
    println!("{}", String::from(ProviderConfig::DummyProviderConfig));
    println!("{}", String::from(ProviderConfig::OpenWeatherConfig));
    Ok(())
}
