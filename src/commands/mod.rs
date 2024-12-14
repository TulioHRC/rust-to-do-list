use clap::Subcommand;
use rusqlite::Connection;
use crate::db::task::{delete_task, insert_task, read_tasks, update_task_status};

use super::Args;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Add a new task to the database
    Add {
        /// Task's name
        #[arg(short = 'n', long = "name")]
        name: String,
    },

    /// Mark a task as done or undone
    Update {
        /// Id of the task
        #[arg(short = 'i', long = "id")]
        id: u32,

        /// Set task to done or not done
        #[arg(short = 'd', long = "done")]
        done: String,
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
      Commands::Update { id, done } => {
        println!("Update task {id} with done status: {done}");
        let done = match done.to_lowercase().as_str() {
          "true" => true,
          "false" => false,
          _ => {
            eprintln!("Invalid value for done status. Expected 'true' or 'false'");
            return;
          }
        };
        match update_task_status(conn, id, done) {
          Ok(task) => {
            println!("Task updated successfully: {}", task.name);
            task.log();
          }
          Err(err) => {
            eprintln!("Error updating task: {}", err);
          }
        }
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

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::connect_db;
  use rusqlite::Result;

  fn setup() -> Result<Connection>{
    // Set up test environment
    return connect_db(Some(true));
  }

  #[test]
  fn test_add_task() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task")
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();

    assert!(
      tasks.len() == 1 && tasks[0].name == String::from("Test Task"),
      "The task was not added to the database"
    );
  }

  #[test]
  fn test_update_task_status_true() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task")
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();
    let task = &tasks[0];

    let args = Args {
      cmd: Commands::Update {
        id: task.id,
        done: String::from("true"),
      },
    };
    
    command_switch(args, &conn);
    
    let updated_task = read_tasks(&conn).unwrap().into_iter().find(|t| t.id == task.id).unwrap();

    assert!(
      updated_task.is_done == true,
      "The task status was not updated"
    );
  }

  #[test]
  fn test_update_task_status_false() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task")
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();
    let task = &tasks[0];

    let args = Args {
      cmd: Commands::Update {
        id: task.id,
        done: String::from("true"),
      },
    };

    command_switch(args, &conn);

    let updated_task = read_tasks(&conn).unwrap().into_iter().find(|t| t.id == task.id).unwrap();

    assert!(
      updated_task.is_done == true,
      "The task status was not updated to true"
    );
    
    let args = Args {
      cmd: Commands::Update {
        id: task.id,
        done: String::from("false"),
      },
    };
    
    command_switch(args, &conn);
    
    let updated_task = read_tasks(&conn).unwrap().into_iter().find(|t| t.id == task.id).unwrap();

    assert!(
      updated_task.is_done == false,
      "The task status was not updated to false"
    );
  }

  #[test]
  fn test_update_task_done_error() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task")
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();
    let task = &tasks[0];

    let args = Args {
      cmd: Commands::Update {
        id: task.id,
        done: String::from("invalid"),
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();

    assert!(
      tasks.len() == 1 && tasks[0].is_done == false,
      "The task status was not updated to false with invalid input"
    );
  }

  #[test]
  fn test_delete_task() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task")
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();
    let task = &tasks[0];

    let args = Args {
      cmd: Commands::Delete {
        id: task.id,
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();

    assert!(
      tasks.len() == 0,
      "The task was not deleted from the database"
    );
  }

  #[test]
  fn test_get_tasks() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task 1"),
      },
    };
    
    command_switch(args, &conn);
    
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task 2"),
      },
    };
    
    command_switch(args, &conn);
    
    let args = Args {
      cmd: Commands::Get {},
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();

    assert!(
      tasks.len() == 2,
      "The tasks were not listed"
    );
  }
}
