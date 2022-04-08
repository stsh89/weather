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
    }
}

#[cfg(test)]
mod tests {
    use super::show;
    use crate::forecast::{Error, Request, Weather};
    use crate::providers::DummyProvider;

    #[test]
    fn it_returns_unknown_error() {
        let request = Request {
            latitude: 0.0,
            longitude: 0.0,
        };
        let result = show(Box::new(DummyProvider), request);

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
        let result = show(Box::new(DummyProvider), request);
        let want = Weather { temperature: 10.22 };

        match result {
            Ok(got) => {
                assert_eq!(got.temperature, want.temperature)
            }
            Err(_) => unreachable!(),
        }
    }
}
