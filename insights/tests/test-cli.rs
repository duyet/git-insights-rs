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

    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg(temp_dir_path)
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_from_github_url() {
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_from_multiline_git_folders() {
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    // Run git clone
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/duyet/git-insights-rs.git")
        .arg("project_1")
        .current_dir(temp_dir_path)
        .assert()
        .success();

    Command::new("git")
        .arg("clone")
        .arg("https://github.com/duyet/git-insights-rs.git")
        .arg("project_2")
        .current_dir(temp_dir_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg(temp_dir_path)
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_from_multiple_two_local_dirs() {
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    // Run git clone
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/duyet/git-insights-rs.git")
        .arg(temp_dir_path)
        .assert()
        .success();

    // $ insights /tmp/xxx /tmp/xxx /tmp/xxx
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg(temp_dir_path)
        .arg(temp_dir_path)
        .arg(temp_dir_path)
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_from_multiple_local_dir_and_remote_url() {
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    let github_url = "https://github.com/duyet/git-insights-rs.git";

    // Run git clone
    Command::new("git")
        .arg("clone")
        .arg(github_url)
        .arg(temp_dir_path)
        .assert()
        .success();

    // $ insights /tmp/xxx /tmp/xxx /tmp/xxx
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg(temp_dir_path)
        .arg(github_url)
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_from_multiple_two_remote_urls() {
    let github_url = "https://github.com/duyet/git-insights-rs.git";

    // $ insights https://github.com/duyet/git-insights-rs.git https://github.com/duyet/git-insights-rs.git
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg(github_url)
        .arg(github_url)
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_invalid_git_folder() {
    // Create empty folder
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg(temp_dir_path)
        .assert()
        .failure()
        .stderr(predicates::str::contains("No .git found"));
}
