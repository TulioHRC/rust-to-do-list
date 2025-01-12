use rusqlite::{Connection, Error, Result};

pub struct Task {
  pub id: u32,
  pub name: String,
  pub is_done: bool,
  pub created_at: String,
}

impl Task {
  pub fn log(&self){
    println!("id = {}, name = {}, is_done = {}, created_at = {}",
             self.id, self.name, self.is_done, self.created_at);
  }
}

pub fn build_tasks_db_table (conn: &Connection) -> Result<()> {
  conn.execute(
    "CREATE TABLE IF NOT EXISTS tasks (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
      deleted_at DATETIME NULL,
      name TEXT NOT NULL,
      is_done BOOLEAN NOT NULL
    )",
    [],
  ).unwrap();

  Ok(())
}
pub fn insert_task(conn: &Connection, task_name: String) -> Result<Task> {
  if task_name == "" || task_name.is_empty() {
    println!("Task name cannot be empty");
    return Err(Error::InvalidParameterName("Task name cannot be empty".to_string()));
  }
  let sql = "INSERT INTO tasks (name, is_done) VALUES (?, ?) RETURNING id, name, is_done, created_at";
  let mut statement = conn.prepare(sql)?;

  let inserted_task = statement.query_row(
      [&task_name as &dyn rusqlite::ToSql, &false],
      |row| {
          Ok(Task {
              id: row.get(0)?,
              name: row.get(1)?,
              is_done: row.get(2)?,
              created_at: row.get(3)?,
          })
      },
  )?;

  Ok(inserted_task)
}

pub fn update_task_status(conn: &Connection, id: u32, is_done: bool) -> Result<Task> {
  let sql = "UPDATE tasks SET is_done = ? WHERE id = ?";
  conn.execute(sql, &[&is_done, &id as &dyn rusqlite::ToSql]).unwrap();

  let sql = "SELECT id, name, is_done, created_at FROM tasks WHERE id =?";
  let mut statement = conn.prepare(sql)?;

  let updated_task = statement.query_row(
    &[&id],
    |row| {
      Ok(Task {
        id: row.get(0)?,
        name: row.get(1)?,
        is_done: row.get(2)?,
        created_at: row.get(3)?,
      })
    },
  )?;
  Ok(updated_task)
}

pub fn read_tasks(conn: &Connection) -> Result<Vec<Task>> {
  let sql = format!("SELECT id, name, is_done, created_at FROM tasks WHERE deleted_at is NULL");
  
  let mut statement = conn.prepare(&sql).unwrap();

  let tasks_iter = statement
    .query_map([], |row| {
        Ok(Task {
            id: row.get(0)?, 
            name: row.get(1)?,
            is_done: row.get(2)?,
            created_at: row.get(3)?,
        })
    })
    .unwrap();

  let mut tasks = Vec::new();
  for task in tasks_iter {
    tasks.push(task?);
  }

  Ok(tasks)
}

pub fn delete_task(conn: &Connection, id: u32) -> Result<Task> {
  // Find task
  let sql = "SELECT id, name, is_done, created_at FROM tasks WHERE id =? AND deleted_at IS NULL";
  let mut statement = conn.prepare(sql)?;

  let task = statement.query_row(
    &[&id],
    |row| {
      Ok(Task {
        id: row.get(0)?,
        name: row.get(1)?,
        is_done: row.get(2)?,
        created_at: row.get(3)?,
      })
    },
  );

  match task {
    Ok(task) => {
      task.log();
    },
    Err(_) => {
      println!("Task not found");
      return Err(rusqlite::Error::QueryReturnedNoRows);
    },
  }

  // Delete task
  let sql = "UPDATE tasks SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";
  conn.execute(sql, &[&id]).unwrap();
  let sql = "SELECT id, name, is_done, created_at FROM tasks WHERE id = ? AND deleted_at IS NOT NULL";
  let mut statement = conn.prepare(sql)?;

  let deleted_task = statement.query_row(
    &[&id],
    |row| {
      Ok(Task {
        id: row.get(0)?,
        name: row.get(1)?,
        is_done: row.get(2)?,
        created_at: row.get(3)?,
      })
    },
  )?;

  return Ok(deleted_task)
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::connect_db;

  fn setup() -> Result<Connection>{
    // Set up test environment
    return connect_db(Some(true), None);
  }

  #[test]
  fn test_insert_task() {
    let conn = setup();

    match conn {
      Ok(conn) => {
        let task = Task {
          id: 1,
          name: "Test task".to_string(),
          is_done: false,
          created_at: "".to_string(),
        };

        match insert_task(&conn, "Test task".to_string()) {
          Ok(task_inserted) => {
            println!("Test passed: Inserted task successfully");
            assert!(task_inserted.id == task.id);
            assert!(task_inserted.name == task.name);
            assert!(task_inserted.is_done == task.is_done);
          }
          Err(err) => {
            eprintln!("Failed to insert task: {}", err);
            assert!(false);
          }
        }
      }
      Err(err) => {
        eprintln!("Error connecting to the database: {}", err);
        assert!(false);
      }
    }
  }

  #[test]
  fn test_insert_and_read_single_task() {
    let conn = setup().unwrap();

    let task = insert_task(&conn, "Test task".to_string()).unwrap();

    let tasks = read_tasks(&conn).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, task.id);
    assert_eq!(tasks[0].name, task.name);
    assert_eq!(tasks[0].is_done, task.is_done);
    assert_eq!(tasks[0].created_at, task.created_at);
  }

  #[test]
  fn test_multiple_insert_task_and_read_tasks() {
    let conn = setup().unwrap();

    let task1 = insert_task(&conn, "Test task".to_string()).unwrap();
    let task2 = insert_task(&conn, "Test 2 task".to_string()).unwrap();


    let tasks = read_tasks(&conn).unwrap();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].id, task1.id);
    assert_eq!(tasks[0].name, task1.name);
    assert_eq!(tasks[0].is_done, task1.is_done);
    assert_eq!(tasks[0].created_at, task1.created_at);

    assert_eq!(tasks[1].id, task2.id);
    assert_eq!(tasks[1].name, task2.name);
    assert_eq!(tasks[1].is_done, task2.is_done);
    assert_eq!(tasks[1].created_at, task2.created_at);
  }
 
  #[test]
  fn test_insert_task_with_empty_name() {
    let conn = setup().unwrap();
    
    // Attempt to insert a task with an empty name
    let result = insert_task(&conn, "".to_string());
    
    // Assertions
    assert!(result.is_err(), "Expected an error when inserting a task with an empty name.");
  }
  
  #[test]
  fn test_insert_task_with_very_long_name() {
      let conn = setup().unwrap();
  
      // Create a very long task name
      let long_task_name = "a".repeat(300); // Assuming 300 characters is beyond typical limits
  
      // Attempt to insert a task with a very long name
      let result = insert_task(&conn, long_task_name.clone());
  
      // Assertions
      match result {
          Ok(task) => {
              assert_eq!(task.name, long_task_name, "The task name should match the very long name provided.");
          }
          Err(err) => {
              eprintln!("Failed to insert task with a very long name: {}", err);
              assert!(false, "Expected to successfully insert a task with a very long name.");
          }
      }
  }
  
  #[test]
  fn test_insert_task_with_special_characters_in_name() {
      let conn = setup().unwrap();
  
      // Task name with special characters
      let special_characters_name = "!@#$%^&*()_+|}{:?><,./;'[]\\=-`~".to_string();
  
      // Attempt to insert a task with special characters in the name
      let result = insert_task(&conn, special_characters_name.clone());
  
      // Assertions
      match result {
          Ok(task) => {
              assert_eq!(task.name, special_characters_name, "The task name should match the special characters name provided.");
          }
          Err(err) => {
              eprintln!("Failed to insert task with special characters: {}", err);
              assert!(false, "Expected to successfully insert a task with special characters in the name.");
          }
      }
  }
  
  #[test]
  fn test_insert_task_defaults_is_done_to_false() {
      let conn = setup().unwrap();
  
      // Insert a task
      let task_name = "New Task".to_string();
      let result = insert_task(&conn, task_name.clone());
  
      // Assertions
      match result {
          Ok(task) => {
              assert_eq!(task.is_done, false, "The 'is_done' field should be set to false upon insertion.");
          }
          Err(err) => {
              eprintln!("Failed to insert task: {}", err);
              assert!(false, "Expected to successfully insert a task.");
          }
      }
  }
  #[test]
  fn test_update_task_and_read_task() {
      let conn = setup().unwrap();
      let task = insert_task(&conn, "Test task".to_string()).unwrap();

      let updated_task = update_task_status(&conn, task.id, true).unwrap();

      assert_eq!(updated_task.is_done, true);
      assert_eq!(updated_task.id, task.id);
      assert_eq!(updated_task.name, task.name);
  }

  #[test]
  fn test_update_fails_for_nonexistent_task() {
      let conn = setup().unwrap();

      let result = update_task_status(&conn, 999, true);
      assert!(result.is_err(), "Expected error when updating nonexistent task");
  }

  #[test]
  fn test_update_task_status_no_change() {
      let conn = setup().unwrap();
      let task = insert_task(&conn, "Test task".to_string()).unwrap();

      // Update to the same status
      let updated_task = update_task_status(&conn, task.id, false).unwrap();

      assert_eq!(updated_task.is_done, false);
      assert_eq!(updated_task.id, task.id);
      assert_eq!(updated_task.name, task.name);
  }

  #[test]
  fn test_update_task_status_twice() {
    let conn = setup().unwrap();
    let task = insert_task(&conn, "Test task".to_string()).unwrap();

    // Update to the same status
    update_task_status(&conn, task.id, true).unwrap();

    // Update to the same status again
    let updated_task_again = update_task_status(&conn, task.id, false).unwrap();

    assert_eq!(updated_task_again.is_done, false);
    assert_eq!(updated_task_again.id, task.id);
    assert_eq!(updated_task_again.name, task.name);
  }

  #[test]  
  fn test_update_fails_with_invalid_task_id() {
      let conn = setup().unwrap();

      // Using a task ID of 0 (invalid in many systems)
      let result = update_task_status(&conn, 0, true);
      assert!(result.is_err(), "Expected error when using an invalid task ID of 0");

      // Using a large task ID that doesn't exist
      let result = update_task_status(&conn, u32::MAX, true);
      assert!(result.is_err(), "Expected error when using a very large invalid task ID");
  }

  #[test]
  fn test_read_tasks_returns_empty_when_no_tasks_exist() {
      let conn = setup().unwrap();
      let tasks = read_tasks(&conn).unwrap();
      assert!(tasks.is_empty(), "Expected no tasks but found some");
  }

  #[test]
  fn test_read_tasks_returns_empty_for_empty_db() {
      let conn = setup().unwrap();

      let tasks = read_tasks(&conn).unwrap();
      assert!(tasks.is_empty(), "Expected no tasks, but some tasks were found.");
  }

  #[test]
  fn test_read_tasks_returns_single_task() {
      let conn = setup().unwrap();

      // Insert a single task
      let task = insert_task(&conn, "Single Task".to_string()).unwrap();

      // Read tasks
      let tasks = read_tasks(&conn).unwrap();

      // Assertions
      assert_eq!(tasks.len(), 1, "Expected 1 task, but found {}", tasks.len());
      let retrieved_task = &tasks[0];
      assert_eq!(retrieved_task.id, task.id);
      assert_eq!(retrieved_task.name, task.name);
      assert_eq!(retrieved_task.is_done, task.is_done);
  }

  #[test]
  fn test_read_tasks_returns_multiple_tasks() {
      let conn = setup().unwrap();

      // Insert multiple tasks
      let task1 = insert_task(&conn, "Task 1".to_string()).unwrap();
      let task2 = insert_task(&conn, "Task 2".to_string()).unwrap();

      // Read tasks
      let tasks = read_tasks(&conn).unwrap();

      // Assertions
      assert_eq!(tasks.len(), 2, "Expected 2 tasks, but found {}", tasks.len());
      assert!(tasks.iter().any(|task| task.id == task1.id && task.name == task1.name));
      assert!(tasks.iter().any(|task| task.id == task2.id && task.name == task2.name));
  }

  #[test]
  fn test_read_tasks_excludes_deleted_entries() {
      let conn = setup().unwrap();

      // Insert an active task
      let active_task = insert_task(&conn, "Active Task".to_string()).unwrap();

      // Insert a task and mark it as deleted
      let deleted_task = insert_task(&conn, "Deleted Task".to_string()).unwrap();
      conn.execute(
          "UPDATE tasks SET deleted_at = datetime('now') WHERE id = ?",
          &[&deleted_task.id],
      )
      .unwrap();

      // Read tasks
      let tasks = read_tasks(&conn).unwrap();

      // Assertions
      assert_eq!(tasks.len(), 1, "Expected 1 active task, but found {}", tasks.len());
      let retrieved_task = &tasks[0];
      assert_eq!(retrieved_task.id, active_task.id);
      assert_eq!(retrieved_task.name, active_task.name);
      assert_eq!(retrieved_task.is_done, active_task.is_done);
  }

  #[test]
  fn test_read_tasks_handles_incomplete_and_complete_states() {
      let conn = setup().unwrap();

      // Insert tasks with different states
      let incomplete_task = insert_task(&conn, "Incomplete Task".to_string()).unwrap();
      let completed_task = insert_task(&conn, "Completed Task".to_string()).unwrap();
      conn.execute(
          "UPDATE tasks SET is_done = 1 WHERE id = ?",
          &[&completed_task.id],
      )
      .unwrap();
      let deleted_task = insert_task(&conn, "Deleted Task".to_string()).unwrap();
      conn.execute(
          "UPDATE tasks SET deleted_at = datetime('now') WHERE id = ?",
          &[&deleted_task.id],
      )
      .unwrap();

      // Read tasks
      let tasks = read_tasks(&conn).unwrap();

      // Assertions
      assert_eq!(tasks.len(), 2, "Expected 2 active tasks, but found {}", tasks.len());
      assert!(tasks.iter().any(|task| task.id == incomplete_task.id && !task.is_done));
      assert!(tasks.iter().any(|task| task.id == completed_task.id && task.is_done));
  }

  #[test]
  fn test_delete_fails_for_nonexistent_task() {
      let conn = setup().unwrap();

      // Attempt to delete a non-existent task
      let result = delete_task(&conn, 999);

      // Assertions
      assert!(result.is_err(), "Expected an error when deleting a non-existent task.");
  }

  #[test]
  fn test_delete_task_is_idempotent() {
      let conn = setup().unwrap();

      // Insert a task
      let task = insert_task(&conn, "Task to be deleted multiple times".to_string()).unwrap();

      // Delete the task once
      delete_task(&conn, task.id).unwrap();

      // Attempt to delete the task again
      let result = delete_task(&conn, task.id);

      // Assertions
      assert!(
          result.is_err(),
          "Expected an error when deleting an already deleted task."
      );

      // Verify the task is still marked as deleted
      let mut statement = conn
          .prepare("SELECT deleted_at FROM tasks WHERE id = ?")
          .unwrap();
      let deleted_at: Option<String> = statement.query_row([&task.id], |row| row.get(0)).unwrap();
      assert!(deleted_at.is_some(), "Expected task to remain marked as deleted.");
  }

  #[test]
  fn test_delete_task_does_not_affect_other_tasks() {
        let conn = setup().unwrap();

        // Insert multiple tasks
        let task1 = insert_task(&conn, "Task 1".to_string()).unwrap();
        let task2 = insert_task(&conn, "Task 2".to_string()).unwrap();

        // Delete one of the tasks
        delete_task(&conn, task1.id).unwrap();

        // Verify the other task is not deleted
        let mut statement = conn
            .prepare("SELECT deleted_at FROM tasks WHERE id = ?")
            .unwrap();
        let deleted_at: Option<String> = statement.query_row([&task2.id], |row| row.get(0)).unwrap();
        assert!(
            deleted_at.is_none(),
            "Expected other tasks to remain unaffected by deletion."
        );
    }

  #[test]
  fn test_delete_task_marks_as_deleted() {
    let conn = setup().unwrap();

    // Insert a task
    let task = insert_task(&conn, "Task to be deleted".to_string()).unwrap();

    // Delete the task
    delete_task(&conn, task.id).unwrap();

    // Verify the task is deleted
    let mut statement = conn
       .prepare("SELECT deleted_at FROM tasks WHERE id = ?")
       .unwrap();
    let deleted_at: Option<String> = statement.query_row([&task.id], |row| row.get(0)).unwrap();
    assert!(deleted_at.is_some(), "Expected task to be marked as deleted.");
  }
}