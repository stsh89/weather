use super::{process_provider_result, ForecastError, Weather};
use crate::providers::Provider;

pub fn run(
    provider: &dyn Provider,
    address_string: &str,
    timestamp: i64,
) -> Result<Weather, ForecastError> {
    let result = provider.daily(address_string, timestamp);
    process_provider_result(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::DummyProvider;

    #[test]
    fn it_returns_unknown_error() {
        let timestamp = 0;
        let provider = DummyProvider::default();
        let result = run(&provider, "Paris,ZZ", timestamp);

        match result {
            Err(ForecastError::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_invalid_address_format_error() {
        let timestamp = 0;
        let provider = DummyProvider::default();

        let result = run(&provider, "", timestamp);

        match result {
            Err(ForecastError::InvalidAddressFormat) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_unauthorized_error() {
        let provider = DummyProvider::default();
        let timestamp = 0;
        let result = run(&provider, "Paris,UU", timestamp);

        match result {
            Err(ForecastError::UnauthorizedProvider) => {}
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

        let result = run(&provider, "", timestamp);

        match result {
            Err(ForecastError::ProviderIsNotValid) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_missing_requested_date_erro() {
        let timestamp = 0;
        let provider = DummyProvider::default();
        let result = run(&provider, "Paris,YY", timestamp);

        match result {
            Err(ForecastError::MissingRequestedDate) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_weather() {
        let timestamp = 0;
        let provider = DummyProvider::default();
        let result = run(&provider, "Paris,FR", timestamp);
        let want = Weather { temperature: 10.22 };

        match result {
            Ok(got) => {
                assert_eq!(got.temperature, want.temperature)
            }
            Err(_) => unreachable!(),
        }
    }
}
