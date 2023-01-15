use assert_cmd::prelude::*;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn parse_git_folder() {
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    // Run git clone
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/duyet/athena-rs.git")
        .arg(temp_dir_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("numstat_parser").unwrap();
    cmd.arg(temp_dir_path)
        .assert()
        .success()
        .stdout(predicates::str::contains("commit"))
        .stdout(predicates::str::contains("author"))
        .stdout(predicates::str::contains("date"))
        .stdout(predicates::str::contains("message"))
        .stdout(predicates::str::contains("Numstat"));
}

#[test]
fn parse_invalid_git_folder() {
    // Create empty folder
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("numstat_parser").unwrap();
    cmd.arg(temp_dir_path)
        .assert()
        .failure()
        .stderr(predicates::str::contains("No .git found"));
}
