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
fn parse_from_github_url_with_year() {
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--year=2023")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("Duyet Le"))
        .stdout(predicates::str::contains("language"))
        .stdout(predicates::str::contains("rs"));
}

#[test]
fn parse_from_github_url_with_author() {
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--author=duyetbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("duyetbot"))
        .stdout(predicates::str::contains("Duyet Le").count(0));
}

#[test]
fn parse_from_github_url_with_ignore_author() {
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--ignore-author")
        .arg("duyetbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("duyetbot").count(0));
}

#[test]
fn parse_from_github_url_with_ignore_ext() {
    // First: it is containing .yaml file
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml"));

    // Ignore --ignore-ext yaml
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--ignore-ext")
        .arg("yaml")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml").count(0));
}

#[test]
fn parse_from_github_url_with_remap_name() {
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-name")
        .arg("duyetbot=>duyetsuperbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("duyetbot").count(0))
        .stdout(predicates::str::contains("duyetsuperbot"));
}

#[test]
fn parse_from_github_url_with_remap_name_multiple() {
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-name")
        .arg("duyetbot,duyet=>duyetsuperbot")
        .assert()
        .success()
        .stdout(predicates::str::contains("Commit by authors"))
        .stdout(predicates::str::contains("duyetbot").count(0))
        .stdout(predicates::str::contains("duyetsuperbot"));
}

#[test]
fn parse_from_github_url_with_remap_ext() {
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-ext")
        .arg("yaml=>lmay")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml").count(0))
        .stdout(predicates::str::contains("lmay"));

    let mut cmd = Command::cargo_bin("insights").unwrap();
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
    let mut cmd = Command::cargo_bin("insights").unwrap();
    cmd.arg("https://github.com/duyet/git-insights-rs.git")
        .arg("--remap-ext")
        .arg("yaml,yml=>duet")
        .assert()
        .success()
        .stdout(predicates::str::contains("yaml").count(0))
        .stdout(predicates::str::contains("duet"));
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
