use std::process::Command;
use std::str;

#[test]
fn test_todo_list_empty() {
    let output = Command::new(env!("CARGO_BIN_EXE_todo"))
        .arg("list")
        .output()
        .expect("Failed to execute todo binary");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    assert!(output.status.success());
    assert_ne!(stdout.trim(), "");

    // println!("STDOUT:\n{}", stdout);
}
