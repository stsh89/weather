use super::{ForecastError, Request, Weather};
use crate::providers::{Provider, ProviderError};

pub fn run(
    provider: &dyn Provider,
    request: &Request,
    timestamp: i64,
) -> Result<Weather, ForecastError> {
    let result = provider.daily(request.latitude, request.longitude, timestamp);

    match result {
        Ok(weather) => Ok(weather),
        Err(ProviderError::Unknown) => Err(ForecastError::Unknown),
        Err(ProviderError::InvalidConfiguration) => Err(ForecastError::ProviderIsNotValid),
        Err(ProviderError::MissingRequestedDate) => Err(ForecastError::MissingRequestedDate),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::DummyProvider;

    #[test]
    fn it_returns_unknown_error() {
        let timestamp = 0;
        let provider = DummyProvider::default();
        let request = Request {
            latitude: 0.0,
            longitude: 0.0,
        };
        let result = run(&provider, &request, timestamp);

        match result {
            Err(ForecastError::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_provider_is_not_valid_error() {
        let timestamp = 0;
        let provider = DummyProvider {
            latitude: None,
            longitude: None,
        };
        let request = Request {
            latitude: 0.0,
            longitude: 0.0,
        };

        let result = run(&provider, &request, timestamp);

        match result {
            Err(ForecastError::ProviderIsNotValid) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_missing_requested_date_erro() {
        let timestamp = 0;
        let provider = DummyProvider::default();
        let request = Request {
            latitude: 0.2,
            longitude: 0.2,
        };
        let result = run(&provider, &request, timestamp);

        match result {
            Err(ForecastError::MissingRequestedDate) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_weather() {
        let timestamp = 0;
        let provider = DummyProvider::default();
        let request = Request {
            latitude: 0.1,
            longitude: 0.1,
        };
        let result = run(&provider, &request, timestamp);
        let want = Weather { temperature: 10.22 };

        match result {
            Ok(got) => {
                assert_eq!(got.temperature, want.temperature)
            }
            Err(_) => unreachable!(),
        }
    }
}
