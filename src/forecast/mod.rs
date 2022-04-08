mod request;
mod weather;

use crate::providers::{Error as ProviderError, Provider};
pub use request::Request;
pub use weather::Weather;

#[derive(Debug)]
pub enum Error {
    Unknown,
}

pub fn show(provider: Box<dyn Provider>, request: Request) -> Result<Weather, Error> {
    let result = provider.provide(request.latitude, request.longitude);

    match result {
        Ok(weather) => Ok(weather),
        Err(ProviderError::Unknown) => Err(Error::Unknown),
        Err(ProviderError::InvalidConfiguration) => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::show;
    use crate::forecast::{Error, Request, Weather};
    use crate::providers::{DummyProvider, Provider};

    #[test]
    fn it_returns_unknown_error() {
        let provider = DummyProvider::default();
        let request = Request {
            latitude: 0.0,
            longitude: 0.0,
        };
        assert_eq!(provider.valid(), true);
        let result = show(Box::new(provider), request);

        match result {
            Ok(_) => unreachable!(),
            Err(Error::Unknown) => {}
        }
    }

    #[test]
    fn it_returns_invalid_configuration_error() {
        let provider = DummyProvider {
            latitude: None,
            longitude: None,
        };
        let request = Request {
            latitude: 0.0,
            longitude: 0.0,
        };

        assert_eq!(provider.valid(), false);
        let result = show(Box::new(provider), request);

        match result {
            Ok(_) => unreachable!(),
            Err(Error::Unknown) => {}
        }
    }

    #[test]
    fn it_returns_weather() {
        let request = Request {
            latitude: 0.1,
            longitude: 0.1,
        };
        let result = show(Box::new(DummyProvider::default()), request);
        let want = Weather { temperature: 10.22 };

        match result {
            Ok(got) => {
                assert_eq!(got.temperature, want.temperature)
            }
            Err(_) => unreachable!(),
        }
    }
}
