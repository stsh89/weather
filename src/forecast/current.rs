use super::{ForecastError, Request, Weather};
use crate::providers::{Provider, ProviderError};

pub fn run(provider: &dyn Provider, request: &Request) -> Result<Weather, ForecastError> {
    let result = provider.current(request.latitude, request.longitude);

    match result {
        Ok(weather) => Ok(weather),
        Err(ProviderError::Unknown) => Err(ForecastError::Unknown),
        Err(ProviderError::InvalidConfiguration) => Err(ForecastError::ProviderIsNotValid),
        Err(_) => Err(ForecastError::Unknown),
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
        let result = run(&provider, &request);

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

        let result = run(&provider, &request);

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
        let result = run(&provider, &request);
        let want = Weather { temperature: 10.22 };

        match result {
            Ok(got) => {
                assert_eq!(got.temperature, want.temperature)
            }
            Err(_) => unreachable!(),
        }
    }
}
