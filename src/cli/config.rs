use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DummyProviderConfig {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenWeatherConfig {
    pub appid: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProvidersConfig {
    pub dummy: DummyProviderConfig,
    pub open_weather: OpenWeatherConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub current_provider: String,
    pub providers: ProvidersConfig,
}

impl Default for DummyProviderConfig {
    fn default() -> Self {
        DummyProviderConfig {
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl Default for OpenWeatherConfig {
    fn default() -> Self {
        OpenWeatherConfig {
            appid: "".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            current_provider: String::from(ProviderConfig::DummyProviderConfig),
            providers: ProvidersConfig::default(),
        }
    }
}

#[derive(Debug)]
pub enum ProviderConfig {
    DummyProviderConfig,
    OpenWeatherConfig,
}

impl From<ProviderConfig> for String {
    fn from(t: ProviderConfig) -> Self {
        String::from(match t {
            ProviderConfig::DummyProviderConfig => "dummy",
            ProviderConfig::OpenWeatherConfig => "open_weather",
        })
    }
}

impl TryFrom<&str> for ProviderConfig {
    type Error = ();

    fn try_from(t: &str) -> Result<Self, Self::Error> {
        match t {
            "dummy" => Ok(Self::DummyProviderConfig),
            "open_weather" => Ok(Self::OpenWeatherConfig),
            _ => Err(()),
        }
    }
}

const CONFIG_NAME: &str = "weather";

impl Config {
    pub fn read() -> Config {
        match confy::load(CONFIG_NAME) {
            Ok(config) => config,
            Err(error) => panic!("{:?}", error),
        }
    }

    pub fn write(&self) {
        match confy::store(CONFIG_NAME, self) {
            Ok(_) => (),
            Err(error) => panic!("{:?}", error),
        };
    }
}
