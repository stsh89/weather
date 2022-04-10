use super::{CliError, Config, ProviderConfig};

pub fn run(mut config: Config, name: &str) -> Result<(), CliError> {
    match ProviderConfig::try_from(name.to_string()) {
        Err(_) => Err(CliError::InvalidProviderName),
        _ => {
            config.current_provider = name.to_string();
            config.write();
            Ok(())
        }
    }
}
