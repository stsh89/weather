use super::ProviderConfig;

pub fn run() {
    println!("{}", String::from(ProviderConfig::DummyProviderConfig));
    println!("{}", String::from(ProviderConfig::OpenWeatherConfig));
}
