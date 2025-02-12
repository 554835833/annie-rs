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
  env_logger::builder()
    .filter_level(log::LevelFilter::Info)
    .format_target(false)
    .init();
  println!("hi, I'm annie, welcome to work with me!");
  let cli = Cli::parse_from(args);
  if let Some(command) = cli.command {
    commands::run(command);
  }
}
