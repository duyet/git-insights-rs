use anyhow::{bail, Result};
use log::debug;
use rayon::prelude::*;
use std::path::PathBuf;

use crate::git;
use crate::parse_from_str;

// BUG-023: Helper function for enhanced Git URL validation
fn is_valid_git_url(url: &str) -> bool {
    url.starts_with("https://github.com")
        || url.starts_with("https://gitlab.com")
        || url.starts_with("git@github.com:")
        || url.starts_with("git@gitlab.com:")
        || url.ends_with(".git")
}

pub fn parse_from_path(paths: &[PathBuf]) -> Result<Vec<crate::Numstat>> {
    if paths.len() == 1 {
        let path = &paths[0];

        match path {
            path if path.is_dir() => {
                let dirs = if path.join(".git").exists() {
                    vec![git::get_log(path)?]
                } else {
                    // BUG-020: Safely iterate directory entries
                    let git_dirs = std::fs::read_dir(path)?
                        .filter_map(|entry| {
                            entry.ok().and_then(|e| {
                                let path = e.path();
                                if path.join(".git").exists() {
                                    Some(path)
                                } else {
                                    None
                                }
                            })
                        })
                        .collect::<Vec<_>>();

                    git_dirs
                        .par_iter()
                        .filter_map(|entry| {
                            debug!("running git log on {}", entry.display());
                            git::get_log(entry).ok()
                        })
                        .collect()
                };

                debug!(
                    "Scanning `{}`, found {} git dir(s)",
                    path.display(),
                    dirs.len()
                );

                if dirs.is_empty() {
                    bail!("No .git found");
                }

                let gitlogs = dirs.join("\n");
                debug!("Number of gitlog lines: {}", gitlogs.lines().count());

                parse_from_str(&gitlogs)
            }

            path if path.is_file() => {
                // Parse the file, if the argument is a path to a numstat.txt file
                let output = std::fs::read_to_string(path)?;
                parse_from_str(&output)
            }

            path if is_valid_git_url(&path.to_string_lossy()) => {
                // BUG-023: Enhanced Git URL validation
                let url = path.to_string_lossy();

                // Tempdir to clone
                let temp_dir = tempfile::tempdir()?;
                let temp_dir_path = temp_dir.path().to_path_buf();
                debug!("Created tempdir: {}", temp_dir_path.display());

                // Run git clone
                git::clone(&url, &temp_dir_path)?;

                parse_from_path(&[temp_dir_path])
            }

            invalid_path => {
                bail!("Invalid path: {}", invalid_path.display());
            }
        }
    } else {
        Ok(paths
            .par_iter()
            .flat_map(|path| {
                parse_from_path(&[path.to_path_buf()]).unwrap_or_else(|e| {
                    panic!("Failed to parse path: {}, error: {}", path.display(), e)
                })
            })
            .collect::<Vec<crate::Numstat>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_parse_from_txt_file() {
        let path = Path::new("tests/sample.txt");
        let out = parse_from_path(&[path.to_path_buf()]).unwrap();

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
        let out = parse_from_path(&[path.to_path_buf()]);
        assert!(out.is_err());

        // Check the error message
        assert_eq!(
            out.unwrap_err().to_string(),
            "Invalid path: tests/not_found.txt"
        );
    }

    #[test]
    fn parse_from_real_git_dir() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        // Run git clone
        git::clone(
            "https://github.com/duyet/athena-rs.git",
            &temp_dir_path.to_path_buf(),
        )
        .expect("failed to clone");

        let out = parse_from_path(&[temp_dir_path.to_path_buf()])
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

        let out = parse_from_path(&[temp_dir_path.to_path_buf()]);
        assert!(out.is_err());

        // Error message
        assert_eq!(out.unwrap_err().to_string(), "No .git found".to_string());
    }
}
