use super::{Config, ProviderConfig};

pub fn run(mut config: Config, name: &str) {
    match ProviderConfig::try_from(name.to_string()) {
        Err(_) => println!("Invalid provider name"),
        _ => {
            config.current_provider = name.to_string();
            config.write();
        }
    };
}
