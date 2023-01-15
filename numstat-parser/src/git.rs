use anyhow::Result;
use std::path::PathBuf;

pub fn log_numstat(path: &PathBuf) -> Result<String> {
    let output = std::process::Command::new("git")
        .arg("log")
        .arg("--numstat")
        .arg("--date=rfc")
        .current_dir(path)
        .output()?;

    Ok(String::from_utf8(output.stdout).unwrap())
}
