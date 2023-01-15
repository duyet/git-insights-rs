use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn parse_sample_file() {
    let mut cmd = Command::cargo_bin("numstat_parser").unwrap();
    cmd.arg("tests/sample.txt")
        .assert()
        .success()
        .stdout(predicates::str::contains("commit"))
        .stdout(predicates::str::contains(
            "1d4171694d9322ab22ee7cdd6712b83ecd8ae6c1",
        ))
        .stdout(predicates::str::contains("author"))
        .stdout(predicates::str::contains("date"))
        .stdout(predicates::str::contains("message"))
        .stdout(predicates::str::contains("Numstat"));
}

#[test]
fn parse_file_not_found() {
    let mut cmd = Command::cargo_bin("numstat_parser").unwrap();
    cmd.arg("tests/not-found.txt")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Invalid path"));
}
