mod forecast_error;
mod request;
mod weather;

use crate::providers::{Provider, ProviderError};
pub use forecast_error::ForecastError;
pub use request::Request;
pub use weather::Weather;

pub fn show(provider: &dyn Provider, request: &Request) -> Result<Weather, ForecastError> {
    let result = provider.provide(request.latitude, request.longitude);

    match result {
        Ok(weather) => Ok(weather),
        Err(ProviderError::Unknown) => Err(ForecastError::Unknown),
        Err(ProviderError::InvalidConfiguration) => Err(ForecastError::ProviderIsNotValid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::DummyProvider;

    #[test]
    fn it_returns_unknown_error() {
        let provider = DummyProvider::default();
        let request = Request {
            latitude: 0.0,
            longitude: 0.0,
        };
        let result = show(&provider, &request);

        match result {
            Err(ForecastError::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_provider_is_not_valid_error() {
        let provider = DummyProvider {
            latitude: None,
            longitude: None,
        };
        let request = Request {
            latitude: 0.0,
            longitude: 0.0,
        };

        let result = show(&provider, &request);

        match result {
            Err(ForecastError::ProviderIsNotValid) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_weather() {
        let provider = DummyProvider::default();
        let request = Request {
            latitude: 0.1,
            longitude: 0.1,
        };
        let result = show(&provider, &request);
        let want = Weather { temperature: 10.22 };

        match result {
            Ok(got) => {
                assert_eq!(got.temperature, want.temperature)
            }
            Err(_) => unreachable!(),
        }
    }
}
