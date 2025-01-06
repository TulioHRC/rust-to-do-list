use std::process::Command;
use std::fs;

fn delete_test_db() {
    let file_path = "test_tasks.db";
    
    match fs::remove_file(file_path) {
        Ok(_) => println!("File '{}' deleted!", file_path),
        Err(err) => eprintln!("Error deleting file '{}': {}", file_path, err),
    }
}

fn setup() {
    delete_test_db();
}

#[test]
fn test_cli_add() {
    setup();

    let output = Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test")
        .arg("-t")
        .output()
        .expect("Error adding task");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Add task"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test"));

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_read() {
    setup();

    let output = Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    assert!(output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("1"));

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_no_task_update_error() {
    setup();

    let output = Command::new("cargo")
        .arg("run")
        .arg("update")
        .arg("--id")
        .arg("1")
        .arg("--done")
        .arg("true")
        .arg("-t")
        .output()
        .expect("Error updating task");

    assert!(!output.status.success());

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_no_task_delete_error() {
    setup();

    let output = Command::new("cargo")
        .arg("run")
        .arg("delete")
        .arg("1")
        .arg("-t")
        .output()
        .expect("Error deleting task");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Task not found"));

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_add_read() {
    setup();

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test")
        .arg("-t")
        .output()
        .expect("Error adding task");

    
    let output = Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("is_done = false"));

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_add_multiple_tasks_read() {
    setup();

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 1")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 2")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 3")
        .arg("-t")
        .output()
        .expect("Error adding task");
    
    let output = Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test 1"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test 2"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test 3"));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("is_done = true"));

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_add_multiple_tasks_update() {
    setup();

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 1")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 2")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 3")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("update")
        .arg("--id")
        .arg("1")
        .arg("--done")
        .arg("true")
        .arg("-t")
        .output()
        .expect("Error updating task");

    Command::new("cargo")
        .arg("run")
        .arg("update")
        .arg("--id")
        .arg("3")
        .arg("--done")
        .arg("true")
        .arg("-t")
        .output()
        .expect("Error updating task");
    
    let output = Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test 1, is_done = true"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test 2, is_done = false"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test 3, is_done = true"));

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_add_multiple_tasks_delete() {
    setup();

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 1")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 2")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test 3")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("update")
        .arg("--id")
        .arg("1")
        .arg("--done")
        .arg("true")
        .arg("-t")
        .output()
        .expect("Error updating task");

    Command::new("cargo")
        .arg("run")
        .arg("delete")
        .arg("3")
        .arg("-t")
        .output()
        .expect("Error deleting task");
    
    let output = Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test 1, is_done = true"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test 2, is_done = false"));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("Test 3"));

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_add_read_update() {
    setup();

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    let output = Command::new("cargo")
        .arg("run")
        .arg("update")
        .arg("--id")
        .arg("1")
        .arg("--done")
        .arg("true")
        .arg("-t")
        .output()
        .expect("Error updating task");

    assert!(String::from_utf8_lossy(&output.stdout).contains("Test"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("is_done = true"));

    let output = Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("is_done = true"));

    delete_test_db(); // Tear Down
}

#[test]
fn test_cli_add_read_delete() {
    setup();

    Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test")
        .arg("-t")
        .output()
        .expect("Error adding task");

    Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    let output = Command::new("cargo")
        .arg("run")
        .arg("delete")
        .arg("1")
        .arg("-t")
        .output()
        .expect("Error deleting task");

    assert!(String::from_utf8_lossy(&output.stdout).contains(" 1 "));
    assert!(String::from_utf8_lossy(&output.stdout).contains("deleted"));

    let output = Command::new("cargo")
        .arg("run")
        .arg("get")
        .arg("-t")
        .output()
        .expect("Error reading tasks");

    assert!(output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("Test"));

    delete_test_db(); // Tear Down
}