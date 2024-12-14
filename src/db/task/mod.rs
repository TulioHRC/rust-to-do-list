use rusqlite::{Connection, Result};

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
  match conn.execute(
    "CREATE TABLE IF NOT EXISTS tasks (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
      deleted_at DATETIME NULL,
      name TEXT NOT NULL,
      is_done BOOLEAN NOT NULL
    )",
    [],
  ) {
    Ok(_) => (),
    Err(err) => {
      eprintln!("Error creating tasks table: {}", err);
      return Err(err);
    }
  };

  Ok(())
}
pub fn insert_task(conn: &Connection, task_name: String) -> Result<Task> {
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
  match conn.execute(sql, &[&is_done, &id as &dyn rusqlite::ToSql]) {
    Ok(_) => {
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
    Err(err) => {
      eprintln!("Error updating task status: {}", err);
      Err(err)
    }
  }
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
  let sql = "UPDATE tasks SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";
  match conn.execute(sql, &[&id]) {
    Ok(_) => {
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
    Err(err) => {
      eprintln!("Error deleting task: {}", err);
      return Err(err)
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::connect_db;

  fn setup() -> Result<Connection>{
    // Set up test environment
    return connect_db(Some(true));
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
  fn test_insert_task_and_read_task() {
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
}