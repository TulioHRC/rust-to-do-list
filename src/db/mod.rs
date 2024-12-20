pub mod task;

pub use rusqlite::{Connection, Result};
use task::build_tasks_db_table;

pub fn connect_db(is_test: Option<bool>) -> Result<Connection> {
  let db_path = match is_test {
    Some(true) => ":memory:",
    _ => "tasks.db",
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
    return connect_db(Some(true));
  }

  #[test]
  fn test_connect_and_build_db() {
    let conn = setup();

    match conn {
      Ok(_conn) => {
        println!("Connecting to database working.");
        assert!(true);
      }
      Err(err) => {
        eprintln!("Error connecting to the database: {}", err);
        assert!(false, "Error connecting to database");
      }
    }
  }
}