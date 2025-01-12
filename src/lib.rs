mod commands;
mod db;

use clap::Parser;
pub use crate::db::connect_db;
pub use crate::commands::{Commands, command_switch};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

pub fn run(){
  let args = Args::parse();

  let is_dry_test = match args.cmd {
    Commands::Add { name: _, dry_test } => dry_test,
    Commands::Get { dry_test } => dry_test,
    Commands::Update { id: _, done: _, dry_test } => dry_test,
    Commands::Delete { id: _, dry_test } => dry_test,
  };

  let conn = connect_db(Some(false), 
    Some(is_dry_test)).unwrap();

  command_switch(args, &conn);
}