use super::{process_provider_result, ForecastError, Weather};
use crate::providers::Provider;

pub fn run(provider: &dyn Provider, address_string: &str) -> Result<Weather, ForecastError> {
    let result = provider.current(address_string);
    process_provider_result(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::DummyProvider;

    #[test]
    fn it_returns_unknown_error() {
        let provider = DummyProvider::default();

        let result = run(&provider, "Paris,AA");

        match result {
            Err(ForecastError::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_unauthorized_error() {
        let provider = DummyProvider::default();

        let result = run(&provider, "Paris,UU");

        match result {
            Err(ForecastError::UnauthorizedProvider) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_invalid_address_format_error() {
        let provider = DummyProvider::default();

        let result = run(&provider, "");

        match result {
            Err(ForecastError::InvalidAddressFormat) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_provider_is_not_valid_error() {
        let provider = DummyProvider { is_valid: false };

        let result = run(&provider, "");

        match result {
            Err(ForecastError::ProviderIsNotValid) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_weather() {
        let provider = DummyProvider::default();
        let result = run(&provider, "Paris,FR");
        let want = Weather { temperature: 10.22 };

        match result {
            Ok(got) => {
                assert_eq!(got.temperature, want.temperature)
            }
            Err(_) => unreachable!(),
        }
    }
}
