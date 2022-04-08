use crate::forecast::Weather;

pub enum Error {
    Unknown,
}

pub trait Provider {
    fn get(&self, latitude: f64, longitude: f64) -> Result<Weather, Error>;
}
