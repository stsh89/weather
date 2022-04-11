//! It is a weather CLI for Linux/MacOS, which is responsible for showing weather to a user.
//! The target consumer of the CLI is a human.

mod cli;
mod forecast;
mod geocode;
mod providers;

fn main() {
    cli::run();
}
