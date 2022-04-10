mod config;
mod configure_provider;
mod current_provider;
mod get_weather;
mod list_providers;
mod set_provider;
mod show_provider;

use clap::{Parser, Subcommand};
use config::{Config, DummyProviderConfig, OpenWeatherConfig, ProviderConfig};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ListProviders {},
    ShowProvider { name: String },
    SetProvider { name: String },
    CurrentProvider {},
    Configure { name: String },
    Get { address: String },
}

pub fn run() {
    let config = Config::read();
    let cli = Cli::parse();

    match &cli.command {
        Commands::ListProviders {} => list_providers::run(),
        Commands::ShowProvider { name } => show_provider::run(config, name),
        Commands::SetProvider { name } => set_provider::run(config, name),
        Commands::CurrentProvider {} => current_provider::run(config),
        Commands::Configure { name } => configure_provider::run(config, name),
        Commands::Get { address } => get_weather::run(config, address),
    }
}
