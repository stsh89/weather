use crate::forecast::Weather;

pub enum Error {
    Unknown,
    InvalidConfiguration,
}

pub trait Provider {
    fn provide(&self, latitude: f64, longitude: f64) -> Result<Weather, Error>;
    fn valid(&self) -> bool;
}
