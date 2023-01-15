use anyhow::{bail, Result};
use log::debug;
use rayon::prelude::*;
use std::path::PathBuf;

use crate::git;
use crate::parse_from_str;

pub fn parse_from_path(path: &PathBuf) -> Result<Vec<crate::Numstat>> {
    match path {
        path if path.is_dir() => {
            let dirs = if path.join(".git").exists() {
                vec![git::get_log(path)?]
            } else {
                let git_dirs = std::fs::read_dir(path)?
                    .filter_map(|entry| {
                        let path = entry.expect("could not parse path").path();
                        if path.join(".git").exists() {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                git_dirs
                    .par_iter()
                    .map(|entry| {
                        debug!("Running git log on {}", entry.display());
                        git::get_log(entry).unwrap()
                    })
                    .collect()
            };

            debug!("Scanned {} git dirs", dirs.len());

            if dirs.is_empty() {
                bail!("No .git found");
            }

            let gitlogs = dirs.join("\n");
            debug!("Number of gitlog lines: {}", gitlogs.lines().count());

            parse_from_str(&gitlogs)
        }
        path if path.is_file() => {
            // Parse the file, if the argument is a path to a numstat.txt file
            let output = std::fs::read_to_string(path).unwrap();
            parse_from_str(&output)
        }
        _ => {
            bail!("Invalid path");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::prelude::*;
    use std::path::Path;
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn test_parse_from_txt_file() {
        let path = Path::new("tests/sample.txt");
        let out = parse_from_path(&path.to_path_buf()).unwrap();

        assert_eq!(out.len(), 4);
        assert_eq!(out[0].commit, "1d4171694d9322ab22ee7cdd6712b83ecd8ae6c1");
        assert_eq!(out[1].commit, "598070021341c247fa041baac291fa1bfa0133b4");
        assert_eq!(out[2].commit, "606a3ea6f09604dff3e28d45348bcd3bbf47a1e8");
        assert_eq!(out[3].commit, "920ef0b0ecb95ff653e9d6a4863b64662bb82834");

        // commit 1d4171694d9322ab22ee7cdd6712b83ecd8ae6c1
        // Merge: 32e30ab bbb7c8e
        // Author: Duyet Le <5009534+duyet@users.noreply.github.com>
        // Date:   Tue, 10 Jan 2023 00:35:39 +0700
        //     Merge pull request #42 from duyet/renovate/all-minor-patch
        assert_eq!(out[0].commit, "1d4171694d9322ab22ee7cdd6712b83ecd8ae6c1");
        assert_eq!(out[0].author.name, "Duyet Le");
        assert_eq!(
            out[0].author.email,
            "5009534+duyet@users.noreply.github.com"
        );
        assert_eq!(out[0].date.to_rfc2822(), "Tue, 10 Jan 2023 00:35:39 +0700");
        assert_eq!(out[0].merges, vec!["32e30ab", "bbb7c8e"]);

        // commit 598070021341c247fa041baac291fa1bfa0133b4 (tag: v1)
        // Author: Duyet Le <lvduyet@fossil.com>
        // Date:   Tue, 9 Aug 2022 19:22:31 +0700
        //
        //     feat: build can render from template
        //
        // 1	0	Cargo.toml
        // 33	0	README.md
        // 2	0	examples/base/index.sql
        // ...
        assert_eq!(out[1].commit, "598070021341c247fa041baac291fa1bfa0133b4");
        assert_eq!(out[1].tags, vec!["v1"]);
        assert_eq!(out[1].stats.len(), 15);
        assert_eq!(out[1].stats[0].added, 1);
        assert_eq!(out[1].stats[0].deleted, 0);
        assert_eq!(out[1].stats[0].path, "Cargo.toml");
        assert_eq!(out[1].stats[0].extension, "toml");
    }

    #[test]
    fn parse_from_file_not_found() {
        let path = Path::new("tests/not_found.txt");
        let out = parse_from_path(&path.to_path_buf());
        assert!(out.is_err());

        // Check the error message
        assert_eq!(out.unwrap_err().to_string(), "Invalid path");
    }

    #[test]
    fn parse_from_real_git_dir() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        // Run git clone
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/duyet/athena-rs.git")
            .arg(temp_dir_path)
            .assert()
            .success();

        let out = parse_from_path(&temp_dir_path.to_path_buf())
            .unwrap_or_else(|_| panic!("could not parse git dir {}", temp_dir_path.display()));

        assert!(!out.is_empty());

        // Should contains first commit
        // https://github.com/duyet/athena-rs/commit/920ef0b0ecb95ff653e9d6a4863b64662bb82834
        assert!(out
            .iter()
            .any(|x| x.commit == "920ef0b0ecb95ff653e9d6a4863b64662bb82834"));
    }

    #[test]
    fn parse_from_empty_dir() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        let out = parse_from_path(&temp_dir_path.to_path_buf());
        assert!(out.is_err());

        // Error message
        assert_eq!(out.unwrap_err().to_string(), "No .git found".to_string());
    }
}
