mod cli_app;
mod cli_error;
mod config;
mod configure_provider;
mod current_provider;
mod get_weather;
mod list_providers;
mod set_provider;
mod show_provider;

use cli_app::CliApp;
use cli_error::CliError;

use clap::{Parser, Subcommand};
use config::{Config, OpenWeatherConfig, ProviderConfig, WeatherapiConfig};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available weather providers
    ListProviders {},
    /// Show configuration for selected provider
    ShowProvider {
        name: String,
    },
    /// Set weather provider that will provide weather information
    SetProvider {
        name: String,
    },
    /// Show selected provider
    CurrentProvider {},
    /// Configure weather provider
    Configure {
        name: String,
    },
    /// Get weather information,
    Get {
        address: String,
        date: Option<String>,
    },
}

pub fn run() {
    let config = Config::read();
    let app = CliApp { config };

    let cli = Cli::parse();

    let result: Result<(), CliError> = match &cli.command {
        Commands::ListProviders {} => list_providers::run(),
        Commands::ShowProvider { name } => show_provider::run(&app.config, name),
        Commands::SetProvider { name } => set_provider::run(app.config, name),
        Commands::CurrentProvider {} => current_provider::run(&app.config),
        Commands::Configure { name } => configure_provider::run(app.config, name),
        Commands::Get { address, date } => get_weather::run(&app, address, date),
    };

    match result {
        Ok(()) => print!("\n"),
        Err(CliError::MissingCurrentProvider) => println!("Please set a provider"),
        Err(CliError::InvalidProviderName) => println!("Invalid provider name"),
        Err(CliError::AddressNotFound) => println!("Address not found"),
        Err(CliError::InvalidAddressFormat) => println!("Invalid address format, it should be in a format of CITY,COUNTRY_ALPHA_2_CODE, for example Paris,FR"),
        Err(CliError::Unknown) => println!("Unknown error occured, please try again later."),
        Err(CliError::InvalidCountryCode) => println!("Invalid country code, it consists of two chars, check ISO 3166 for more infomation."),
        Err(CliError::InvalidDateFormat) => println!("Invalid date format, date should be in a format of Yyyy-mm-dd, for example 2022-04-11"),
        Err(CliError::MissingRequestedDate) => println!("Missing forecast for requested date"),
        Err(CliError::ProviderIsNotConfigured) => println!("You should configure provider"),
    }
}
