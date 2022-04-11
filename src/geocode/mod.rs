mod address;
mod client;
mod dummy_client;
mod geocode_error;
mod open_weather_client;
mod point;
mod search_by_address;

pub use address::Address;
pub use client::Client;
pub use geocode_error::GeocodeError;
pub use open_weather_client::OpenWeatherClient;
pub use point::Point;
pub use search_by_address::run as search_by_address;

use client::ClientResponse;
// use dummy_client::DummyClient;
