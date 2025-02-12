use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct MonorepoPackageListCommandArgs {
  pub name: Option<String>,
}

#[derive(Subcommand)]
pub enum MonorepoPackageCommands {
  List(MonorepoPackageListCommandArgs),
}

pub fn run(args: MonorepoPackageListCommandArgs) {
  println!("list");
  println!("name: {:?}", args.name);
}
