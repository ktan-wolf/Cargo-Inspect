use clap::Parser;

#[derive(Parser , Debug)]
#[command(name = "cargo-inspect")]
#[command(about = "Inspect your rust project dependencies , safety and more.")]
pub struct Cli{
    pub command: String,
}
