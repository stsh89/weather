use super::Client;

pub struct Config {
    pub client: Box<dyn Client>,
}
