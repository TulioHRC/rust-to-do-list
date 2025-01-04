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

        /// Dry run test
        #[arg(short = 't', long = "dry-test", default_value_t = false)]
        dry_test: bool,
    },

    /// Mark a task as done or undone
    Update {
        /// Id of the task
        #[arg(short = 'i', long = "id")]
        id: u32,

        /// Set task to done or not done
        #[arg(short = 'd', long = "done")]
        done: String,

        /// Dry run test
        #[arg(short = 't', long = "dry-test", default_value_t = false)]
        dry_test: bool,
    },

    /// List tasks
    Get {
        /// Dry run test
        #[arg(short = 't', long = "dry-test", default_value_t = false)]
        dry_test: bool,
    },

    /// Delete a task by id
    Delete {
        /// Id of the expense
        #[arg()]
        id: u32,

        /// Dry run test
        #[arg(short = 't', long = "dry-test", default_value_t = false)]
        dry_test: bool,
    },
}

pub fn command_switch(args: Args, conn: &Connection) {
    match args.cmd {
      Commands::Add { name, dry_test } => {
        println!("Add task {name} {0}", match dry_test{
          true => "in dry run mode",
          false => "in normal mode"
        });
        insert_task(conn, name).unwrap();
      },
      Commands::Update { id, done, dry_test } => {
        println!("Update task {id} with done status: {done} {0}", match dry_test{
          true => "in dry run mode",
          false => "in normal mode"
        });
        let done = match done.to_lowercase().as_str() {
          "true" => true,
          "false" => false,
          _ => {
            eprintln!("Invalid value for done status. Expected 'true' or 'false'");
            return;
          }
        };
        let updated_task = update_task_status(conn, id, done).unwrap();
        updated_task.log();
      },
      Commands::Get { dry_test } => {
        println!("List tasks {0}", match dry_test{
          true => "in dry run mode",
          false => "in normal mode"
        });
        let tasks = read_tasks(conn).unwrap();
        for task in tasks {
          task.log();
        }
      },
      Commands::Delete { id, dry_test } => {
        println!("Delete task {id} with id {0}", match dry_test{
          true => "in dry run mode",
          false => "in normal mode"
        });
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
    return connect_db(Some(true), None);
  }

  #[test]
  fn test_command_add_task() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task"),
        dry_test: false,
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
  fn test_command_update_task_status_to_true() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task"),
        dry_test: false,
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();
    let task = &tasks[0];

    let args = Args {
      cmd: Commands::Update {
        id: task.id,
        done: String::from("true"),
        dry_test: false,
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
  fn test_command_update_task_status_to_false() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task"),
        dry_test: false,
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();
    let task = &tasks[0];

    let args = Args {
      cmd: Commands::Update {
        id: task.id,
        done: String::from("true"),
        dry_test: false,
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
        dry_test: false,
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
  fn test_command_update_task_with_invalid_done_value() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task"),
        dry_test: false,
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();
    let task = &tasks[0];

    let args = Args {
      cmd: Commands::Update {
        id: task.id,
        done: String::from("invalid"),
        dry_test: false,
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
  fn test_command_delete_nonexistent_task() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Delete {
        id: 1,
        dry_test: false,
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();

    assert!(
      tasks.len() == 0,
      "The non-existent task was not deleted from the database"
    );
  }

  #[test]
  fn test_command_delete_existing_task() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task"),
        dry_test: false,
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();
    let task = &tasks[0];

    let args = Args {
      cmd: Commands::Delete {
        id: task.id,
        dry_test: false,
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
  fn test_command_get_tasks() {
    let conn = setup().unwrap();
      
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task 1"),
        dry_test: false,
      },
    };
    
    command_switch(args, &conn);
    
    let args = Args {
      cmd: Commands::Add {
        name: String::from("Test Task 2"),
        dry_test: false,
      },
    };
    
    command_switch(args, &conn);
    
    let args = Args {
      cmd: Commands::Get {
        dry_test: false,
      },
    };
    
    command_switch(args, &conn);
    
    let tasks = read_tasks(&conn).unwrap();

    assert!(
      tasks.len() == 2,
      "The tasks were not listed"
    );
  }
}
