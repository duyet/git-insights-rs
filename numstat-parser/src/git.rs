use anyhow::{anyhow, Context, Result};
use log::debug;
use std::path::PathBuf;
use std::process::Command;

pub fn clone(url: &str, path: &PathBuf) -> Result<()> {
    debug!("Cloning {} to {}", url, path.display());

    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .output()
        .context("Failed to run git clone")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to run git clone: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn get_log(path: &PathBuf) -> Result<String> {
    // Change current directory to the git repo
    std::env::set_current_dir(path).context("Failed to change current directory")?;

    let cmd = "git log --all --numstat --date=rfc";

    debug!("Running {}", cmd);
    let output = Command::new("git")
        .arg("log")
        .arg("--all")
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_git_clone() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();

        // Run git clone
        clone("https://github.com/duyet/athena-rs.git", &temp_dir_path).unwrap();

        // Check inside the folder, should have the README.md file
        assert!(temp_dir_path.join("README.md").exists());
    }

    #[test]
    fn test_git_log() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();

        // Run git clone
        clone("https://github.com/duyet/athena-rs.git", &temp_dir_path).unwrap();

        // Run git log
        let output = get_log(&temp_dir_path).unwrap();

        // Check the output

        // Should have at least 1 commit contains `README.md` file
        assert!(output.contains("README.md"));
        // Should have at least 1 commit contains `commit` string
        assert!(output.contains("commit"));
    }
}
