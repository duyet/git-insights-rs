use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn parse_from_github_url() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by author"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_from_github_url_with_year() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--year=2023")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by author"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_from_github_url_with_author() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--author=duyetbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by author"))
        .stdout(predicates::str::contains("duyetbot"))
        .stdout(predicates::str::contains("Duyet Le").count(0));
}

#[test]
fn parse_from_github_url_with_ignore_author() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--ignore-author")
        .arg("duyetbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by author"))
        .stdout(predicates::str::contains("duyetbot").count(0));
}

#[test]
fn parse_from_github_url_with_ignore_ext() {
    // First: it is containing .yaml file
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml"));

    // Ignore --ignore-ext yaml
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--ignore-ext")
        .arg("yaml")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml").count(0));
}
