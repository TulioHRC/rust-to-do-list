use std::process::Command;
use std::fs;

fn delete_test_db() {
    let file_path = "test_tasks.db";

    match fs::remove_file(file_path) {
        Ok(_) => println!("File '{}' deleted!", file_path),
        Err(err) => eprintln!("Error deleting file '{}': {}", file_path, err),
    }
}

#[test]
fn test_cli_add_read() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("add")
        .arg("--name")
        .arg("Test")
        .arg("-t")
        .output()
        .expect("Error adding task");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Test"));

    delete_test_db(); // Tear Down
}
