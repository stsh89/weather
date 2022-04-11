use super::CliError;
use super::Config;

pub fn run(config: &Config) -> Result<(), CliError> {
    println!("{}", config.current_provider);
    Ok(())
}
