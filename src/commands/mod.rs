use clap::Subcommand;
use rusqlite::Connection;
use super::Args;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Add a new task to the database
    Add {
        /// Task's name
        #[arg(short = 'n', long = "name")]
        name: String,
    },

    /// List tasks
    Get {},

    /// Delete a task by id
    Delete {
        /// Id of the expense
        #[arg()]
        id: u32,
    },
}

pub fn command_switch(args: Args, _conn: &Connection) {
    match args.cmd {
      Commands::Add { name } => {
        println!("Add task {name}");
      },
      Commands::Get { } => {
        println!("List tasks");
      },
      Commands::Delete { id } => {
        println!("Delete task {id} with id");
      }
    }
}