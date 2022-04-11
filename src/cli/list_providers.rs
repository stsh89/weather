use super::CliError;
use super::ProviderConfig;

pub fn run() -> Result<(), CliError> {
    println!("{}", String::from(ProviderConfig::DummyProvider));
    println!("{}", String::from(ProviderConfig::OpenWeather));
    println!("{}", String::from(ProviderConfig::Weatherapi));

    Ok(())
}
