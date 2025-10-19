use clap::{Parser , Subcommand};

#[derive(Parser , Debug)]
#[command(name = "cargo-inspect")]
#[command(about = "Inspect your rust project dependencies , safety and more.")]
pub struct Cli{
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand , Debug)]
pub enum Commands{
    Deps, 
    Unsafe,
}
