use rusqlite::{Connection, Result};

pub struct Task {
  pub id: u64,
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
  let sql = "INSERT INTO tasks (name) VALUES (?) RETURNING id, name, is_done, created_at";
  let mut statement = conn.prepare(sql)?;

  let inserted_task = statement.query_row(
      [&task_name],
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

pub fn delete_task(conn: &Connection, id: String) -> Result<()> {
  let sql = "UPDATE tasks SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?";
  conn.execute(sql, &[&id]).unwrap();
  Ok(())
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use super::super::connect_db;


//   fn setup() -> Result<Connection>{
//     // Set up test environment
//     return connect_db(Some(true));
//   }

//   #[test]
//   fn test_insert_expense() {
//     let conn = setup();

//     match conn {
//       Ok(conn) => {
//         let expense = Expense {
//           id: 0,
//           name: "Test expense".to_string(),
//           value: 10.50,
//           expense_type: Some("Groceries".to_string()),
//           date: "2022-01-01".to_string(),
//         };

//         match insert_expense(&conn, expense.name, expense.value, expense.expense_type, expense.date) {
//           Ok(_) => {
//             println!("Test passed: Inserted expense successfully");
//           }
//           Err(err) => {
//             eprintln!("Failed to insert expense: {}", err);
//             assert!(false);
//           }
//         }
//       }
//       Err(err) => {
//         eprintln!("Error connecting to the database: {}", err);
//         assert!(false);
//       }
//     }
//   }

//   #[test]
//   fn test_read_expenses(){
//     let conn = setup();

//     match conn {
//       Ok(conn) => {
//         match insert_expense(&conn, 
//           "Test expense 2".to_string(),
//           15.50,
//           Some("Transportation".to_string()),
//           "2022-01-02".to_string(),
//         ) {
//           Ok(_) => {
//             let expenses = read_expenses(&conn, Some(1), Some(2022)).unwrap();
//             if
//               expenses[0].id == 1 && 
//               expenses[0].name == "Test expense 2".to_string() &&
//               expenses[0].value == 15.50 &&
//               expenses[0].expense_type == Some("Transportation".to_string()) &&
//               expenses[0].date == "2022-01-02".to_string()
//             {
//               println!("Expense inserted was found!");
//             } else {
//               eprintln!("Expense inserted was not found!");
//               assert!(false);
//             }
//             assert_eq!(expenses.len(), 1);
//           }
//           Err(err) => {
//             eprintln!("Failed to insert expense: {}", err);
//             assert!(false);
//           }
//         }
//       }
//       Err(err) => {
//         eprintln!("Error connecting to the database: {}", err);
//         assert!(false);
//       }
//     }
//   }

//   #[test]
//   fn test_delete_expense(){
//     let conn = setup();

//     match conn {
//       Ok(conn) => {
//         match insert_expense(&conn, 
//           "Test expense 3".to_string(),
//           20.00,
//           Some("Entertainment".to_string()),
//           "2022-01-03".to_string(),
//         ) {
//           Ok(_) => {
//             delete_expense(&conn, format!("{}", 1)).unwrap();
//             let expenses = read_expenses(&conn, Some(1), Some(2022)).unwrap();
//             assert_eq!(expenses.len(), 0);
//             println!("Test passed: Expense deleted successfully");
//           }
//           Err(err) => {
//             eprintln!("Failed to insert expense: {}", err);
//             assert!(false);
//           }
//         }
//       }
//       Err(err) => {
//         eprintln!("Error connecting to the database: {}", err);
//         assert!(false);
//       }
//     }
//   }

//   #[test]
//   fn test_edit_expense() {
//     let conn = setup();

//     match conn {
//       Ok(conn) => {
//         match insert_expense(&conn, 
//           "Test expense 4".to_string(),
//           25.00,
//           Some("Shopping".to_string()),
//           "2022-01-04".to_string(),
//         ) {
//           Ok(_) => {
//             edit_expense(&conn, &format!("{}", 1), Some(String::from("New expense name")), None, None, None).unwrap();
//             let expenses = read_expenses(&conn, Some(1), Some(2022)).unwrap();
//             if
//               expenses[0].id == 1 && 
//               expenses[0].name == "New expense name".to_string() &&
//               expenses[0].value == 25.00 &&
//               expenses[0].expense_type == Some("Shopping".to_string()) &&
//               expenses[0].date == "2022-01-04".to_string()
//             {
//               println!("Test passed: Expense edited successfully");
//               assert!(true);
//             } else {
//               eprintln!("Expense not found after editing!");
//               assert!(false);
//             }
//           },
//           Err(err) => {
//             eprintln!("Failed to insert expense: {}", err);
//             assert!(false);
//           }
//         }
//       }, 
//       Err(err) => {
//         eprintln!("Error connecting to the database: {}", err);
//         assert!(false);
//       }
//     }
//   }
// }