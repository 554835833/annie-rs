use clap::{command, Parser, Subcommand};
use list::MonorepoPackageListCommandArgs;

pub mod list;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct MonorepoPackageCommandArgs {
  #[command(subcommand)]
  pub command: Option<MonorepoPackageCommands>,
}

#[derive(Subcommand)]
pub enum MonorepoPackageCommands {
  List(MonorepoPackageListCommandArgs),
}

pub fn run(args: MonorepoPackageCommands) {
  match args {
    MonorepoPackageCommands::List(args) => {
      list::run(args);
    }
  }
}
