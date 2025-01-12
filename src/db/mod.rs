pub mod task;

pub use rusqlite::{Connection, Result};
use task::build_tasks_db_table;

pub fn connect_db(is_test: Option<bool>, is_dry_test: Option<bool>) -> Result<Connection> {
  let db_path = match is_test {
    Some(true) => ":memory:",
    _ => match is_dry_test {
      Some(true) => "test_tasks.db",
      _ => "tasks.db"
    }
  };
  let conn = Connection::open(db_path)?;

  build_db(&conn).unwrap();

  return Ok(conn);
}

pub fn build_db(conn: &Connection) -> Result<()> {
  build_tasks_db_table(conn).unwrap();

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn setup() -> Result<Connection>{
    // Set up test environment
    return connect_db(Some(true), None);
  }

  #[test]
  fn test_connect_and_build_db() {
    let conn = connect_db(Some(false), Some(false));
  
    match conn {
      Ok(_conn) => {
        println!("Connecting to real database working.");
        assert!(true);
      }
      Err(err) => {
        eprintln!("Error connecting to the real database: {}", err);
        assert!(false, "Error connecting to real database");
      }
    }
  }

  #[test]
  fn test_connect_and_build_db_test_mode() {
    let conn = setup();

    match conn {
      Ok(_conn) => {
        println!("Connecting to test database working.");
        assert!(true);
      }
      Err(err) => {
        eprintln!("Error connecting to the test database: {}", err);
        assert!(false, "Error connecting to test database");
      }
    }
  }

  #[test]
  fn test_connect_and_build_db_dry_mode() {
    let conn = connect_db(Some(false), Some(true));
  
    match conn {
      Ok(_conn) => {
        println!("Connecting to dry-mode database working.");
        assert!(true);
      }
      Err(err) => {
        eprintln!("Error connecting to the dry-mode database: {}", err);
        assert!(false, "Error connecting to dry-mode database");
      }
    }
  }
}