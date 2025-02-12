use clap::{command, Parser, Subcommand};
use package::MonorepoPackageCommandArgs;

pub mod package;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct MonorepoCommandArgs {
  #[command(subcommand)]
  pub command: Option<MonorepoCommands>,
}

#[derive(Subcommand)]
pub enum MonorepoCommands {
  /// 包管理
  Package(MonorepoPackageCommandArgs),
}

pub fn run(args: MonorepoCommands) {
  match args {
    MonorepoCommands::Package(args) => {
      if let Some(command) = args.command {
        package::run(command);
      }
    }
  }
}
