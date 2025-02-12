#![deny(clippy::all)]
#[macro_use]
extern crate napi_derive;

mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[napi]
pub fn run(args: Vec<String>) {
  let cli = Cli::parse_from(args);
  if let Some(command) = cli.command {
    commands::run(command);
  }
}
