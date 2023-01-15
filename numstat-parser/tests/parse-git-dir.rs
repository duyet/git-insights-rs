use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn parse_sample_file() {
    // Create temp dir and generate sample git project
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    // Run git clone
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/duyet/athena-rs.git")
        .current_dir(temp_dir_path)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("numstat_parser").unwrap();
    cmd.arg(temp_dir_path).assert().success();
}
