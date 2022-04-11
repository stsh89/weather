use super::{ForecastError, Weather};
use crate::providers::{Provider, ProviderError};

pub fn run(provider: &dyn Provider, address_string: &str) -> Result<Weather, ForecastError> {
    let result = provider.current(address_string);

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

        let result = run(&provider, "Paris,ZZ");

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
