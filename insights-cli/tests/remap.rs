use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn parse_from_github_url_with_remap_name() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-name")
        .arg("duyetbot=>duyetsuperbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by author"))
        .stdout(predicates::str::contains("duyetbot").count(0))
        .stdout(predicates::str::contains("duyetsuperbot"));

    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-name")
        .arg("duyetsuperbot<=duyetbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by author"))
        .stdout(predicates::str::contains("duyetbot").count(0))
        .stdout(predicates::str::contains("duyetsuperbot"));
}

#[test]
fn parse_from_github_url_with_remap_name_multiple() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-name")
        .arg("duyetbot,duyet=>duyetsuperbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by author"))
        .stdout(predicates::str::contains("duyetbot").count(0))
        .stdout(predicates::str::contains("duyetsuperbot"));
}

#[test]
fn parse_from_github_url_with_remap_ext() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-ext")
        .arg("yaml=>lmay")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml").count(0))
        .stdout(predicates::str::contains("lmay"));

    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-ext")
        .arg("some_ext<=yaml")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml").count(0))
        .stdout(predicates::str::contains("some_ext"));
}

#[test]
fn parse_from_github_url_with_remap_ext_multiple() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-ext")
        .arg("yaml,yml=>duet")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml").count(0))
        .stdout(predicates::str::contains("duet"));
}

#[test]
fn parse_from_github_url_with_remap_invalid() {
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-ext")
        .arg("yaml,yml=duet")
        .arg("--remap-name")
        .arg("abc")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Invalid remap format"));
}

#[test]
fn parse_from_github_url_with_remap_email() {
    let github_url = "https://github.com/duyet/git-insights-rs.git";

    // $ insights https://github.com/duyet/git-insights-rs.git --remap-email lvduit08@gmail.com=>me@duyet.net
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg(github_url)
        .arg(github_url)
        .arg("--remap-email")
        .arg("lvduit08@gmail.com=>me@duyet.net")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg(github_url)
        .arg(github_url)
        .arg("--remap-email")
        .arg("lvduit08@gmail.com<=me@duyet.net")
        .assert()
        .success();
}

#[test]
fn parse_from_github_url_with_remap_email_multiple() {
    let github_url = "https://github.com/duyet/git-insights-rs.git";

    // $ insights https://github.com/duyet/git-insights-rs.git --remap-email lvduit08@gmail.com=>me@duyet.net
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg(github_url)
        .arg(github_url)
        .arg("--remap-email")
        .arg("lvduit08@gmail.com,abc@gmail.com=>me@duyet.net")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg(github_url)
        .arg(github_url)
        .arg("--remap-email")
        .arg("lvduit08@gmail.com<=me@duyet.net,abc@gmail.com")
        .assert()
        .success();
}
