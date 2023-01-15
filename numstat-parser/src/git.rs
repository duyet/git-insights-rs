use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn get_log(path: &PathBuf) -> Result<String> {
    // Change current directory to the git repo
    std::env::set_current_dir(path).context("Failed to change current directory")?;

    let cmd = "git log --numstat --date=rfc";

    let output = Command::new("git")
        .arg("log")
        .arg("--numstat")
        .arg("--date=rfc")
        .arg(path)
        .current_dir(path)
        .output()
        .with_context(|| format!("Running command: `{}`", cmd))?;

    let stdout = String::from_utf8(output.stdout.clone()).context("Failed to parse stdout")?;
    let stderr = String::from_utf8(output.stderr).context("Failed to parse stderr")?;

    if output.status.success() {
        Ok(stdout)
    } else if stderr.contains("does not have any commits yet") {
        Ok(String::new())
    } else {
        Err(anyhow!(
            "Failed to run command `{}` in `{}`: {:?}",
            cmd,
            path.display(),
            stderr
        ))
    }
}
