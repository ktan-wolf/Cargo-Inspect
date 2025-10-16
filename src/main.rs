mod cli;
mod commands;

use cli::Cli;
use commands::{deps , unsafe_check};

fn main(){
    let cli = Cli::parse();

    match cli.command.as_str(){
        "deps" => deps::run(),
        "unsafe" => unsafe_check::run(),
        _ => println!("Unknown command. Use: cargo inspect [deps|unsafe]")
    }
}
