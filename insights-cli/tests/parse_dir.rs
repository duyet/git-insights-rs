use assert_cmd::prelude::*;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn parse_from_git_folder() {
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    // Run git clone
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/duyet/git-insights-rs.git")
        .arg(temp_dir_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg(temp_dir_path)
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by author"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_invalid_git_folder() {
    // Create empty folder
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg(temp_dir_path)
        .assert()
        .failure()
        .stderr(predicates::str::contains("No .git found"));
}
