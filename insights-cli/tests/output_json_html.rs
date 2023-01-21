use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn output_json() {
    let github_url = "https://github.com/duyet/git-insights-rs.git";

    // $ insights https://github.com/duyet/git-insights-rs.git https://github.com/duyet/git-insights-rs.git
    let mut cmd = Command::cargo_bin("girs").unwrap();
    let out = cmd.arg(github_url).arg("--output=json").assert().success();

    // Get stdout
    let out = &out.get_output().stdout;

    // Parse json
    let json: serde_json::Value =
        serde_json::from_slice(out).expect("could not parse the output as json");
    println!("{:#}", json);

    // Check json
    assert!(!json["commit_by_author"].as_object().unwrap().is_empty());
    assert!(!json["commit_by_author"]["author_name"]
        .as_array()
        .unwrap()
        .is_empty());
}

#[test]
fn output_html() {
    let github_url = "https://github.com/duyet/git-insights-rs.git";

    // $ insights https://github.com/duyet/git-insights-rs.git https://github.com/duyet/git-insights-rs.git
    let mut cmd = Command::cargo_bin("girs").unwrap();
    cmd.arg(github_url)
        .arg("--output=html")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Not implemented yet"));
}
