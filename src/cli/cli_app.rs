use super::Config;
use crate::geocode::Client;

pub struct CliApp {
    pub geocode_client: Box<dyn Client>,
    pub config: Config,
}
