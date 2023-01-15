use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn parse_sample_file() {
    let mut cmd = Command::cargo_bin("numstat_parser").unwrap();
    cmd.arg("tests/sample.txt").assert().success();
}

#[test]
fn parse_file_not_found() {
    let mut cmd = Command::cargo_bin("numstat_parser").unwrap();
    cmd.arg("tests/not-found.txt")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Invalid path"));
}
