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

  let conn = connect_db(Some(false)).unwrap();

  command_switch(args, &conn);
}