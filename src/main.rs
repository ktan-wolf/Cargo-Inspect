mod cli;
mod commands;

use crate::cli::Commands;

use clap::Parser;
use cli::Cli;
use commands::{deps , unsafe_check};

fn main(){
    let cli = Cli::parse();

    match cli.command{
        Commands::Deps => deps::run(),
        Commands::Unsafe => unsafe_check::run(),
    }
}
