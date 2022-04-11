use serde::Deserialize;

use super::GeocodeError;

#[derive(Deserialize)]
pub struct ClientResponse {
    pub lat: f64,
    pub lon: f64,
}

pub trait Client {
    fn search_by_address(&self, q: String) -> Result<ClientResponse, GeocodeError>;
}
