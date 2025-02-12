use clap::Subcommand;
use monorepo::MonorepoCommandArgs;

pub mod monorepo;

#[derive(Subcommand)]
pub enum Commands {
  /// monorepo 管理工具
  Monorepo(MonorepoCommandArgs),
}

pub fn run(args: Commands) {
  match args {
    Commands::Monorepo(monorepo_args) => {
      if let Some(command) = monorepo_args.command {
        monorepo::run(command);
      }
    }
  }
}
