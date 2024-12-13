use clap::Subcommand;
use rusqlite::Connection;
use crate::db::task::{delete_task, insert_task, read_tasks};

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

pub fn command_switch(args: Args, conn: &Connection) {
    match args.cmd {
      Commands::Add { name } => {
        println!("Add task {name}");
        match insert_task(conn, name) {
          Ok(task) => {
            println!("Task added successfully: {}", task.name);
          }
          Err(err) => {
            eprintln!("Error adding task: {}", err);
          }
        };
      },
      Commands::Get { } => {
        println!("List tasks");
        match read_tasks(conn) {
          Ok(tasks) => {
            for task in tasks {
              task.log();
            }
          }
          Err(err) => {
            eprintln!("Error reading tasks: {}", err);
          }
        }
      },
      Commands::Delete { id } => {
        println!("Delete task {id} with id");
        match delete_task(conn, id) {
          Ok(task) => {
            println!("Task deleted successfully");
            task.log();
          }
          Err(err) => {
            eprintln!("Error deleting task: {}", err);
          }
        }
      }
    }
}