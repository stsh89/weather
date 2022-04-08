mod dummy_provider;
mod open_weather;
mod provider;

pub use dummy_provider::DummyProvider;
pub use open_weather::OpenWeather;
pub use provider::{Error, Provider};
