use anyhow::{anyhow, Context, Result};
use chrono::DateTime;
use log::debug;
use rayon::prelude::*;

/// Parse git log --numstat content
///
/// Sample data:
///
/// ```txt
/// commit 4d07e012d8a31d7a19f4c3461d11e0ad83868d6d (HEAD -> chore/ui, origin/chore/ui)
/// Author: Duyet Le <me@duyet.net>
/// Date:   Wed Jan 11 11:22:17 2023 +0700
///
///     fix: timestamp is invalid
///
/// 2       2       config/app_log/summary.ts
/// 9       0       libs/queryBuilder.ts
///
/// commit c5340f86fcf759e26590e795cd82e260f524331f
/// Author: Duyet Le <me@duyet.net>
/// Date:   Wed Jan 11 11:11:32 2023 +0700
///
///     chore: not using clickhouse local table
///
/// 2       2       config/app_log/summary.ts
/// ```
pub fn parse_from_str(s: &str) -> Result<Vec<crate::Numstat>> {
    // Split into block of commits
    let blocks = s
        .split("\ncommit ")
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            if s.trim().starts_with("commit ") {
                s.trim().to_string()
            } else {
                format!("commit {}", s.trim())
            }
        })
        .collect::<Vec<String>>();

    Ok(blocks
        .par_iter()
        .map(|block| parse_block(block).unwrap())
        .collect())
}

fn parse_block(block: &str) -> Result<crate::Numstat> {
    let lines = block.lines();
    let mut numstat = crate::Numstat::default();

    let default_date = numstat.date;

    // Parse line by line
    for line in lines {
        // Skip empty line
        if line.is_empty() {
            continue;
        }

        // Parse commit
        if line.starts_with("commit ") {
            let commit = line.split(' ').nth(1).unwrap();
            numstat.commit = commit.to_string();
            debug!("commit: {}", commit);

            // Bug
            if commit.len() < 15 {
                debug!("{}", block);
                break;
            }

            // commit bbb7c8e78f07cd06dc015c7139cb174285cd6a8c (tag: v1.0.24+demo, HEAD -> master, origin/master)
            let brand_tag_re = regex::Regex::new(r"\((.+)\)").unwrap();
            if let Some(brand_tag) = brand_tag_re.captures(line) {
                let brand_or_tag = brand_tag.get(1).unwrap().as_str();
                numstat.tags = brand_or_tag
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| s.starts_with("tag: "))
                    .map(|s| s.replace("tag: ", ""))
                    .collect();

                // Parse branches
                numstat.branches = brand_or_tag
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.starts_with("tag: ") && !s.starts_with("HEAD -> "))
                    .map(|s| s.to_string())
                    .collect();
            }

            continue;
        }

        // Parse author: Author: Duyet Le <me@duyet.net>
        // TODO: Parse multiple authors
        let author_re = regex::Regex::new(r"Author: (?P<name>.*) <(?P<email>.*)>").unwrap();
        if let Some(captures) = author_re.captures(line) {
            numstat.author.full = line.trim_start_matches("Author: ").to_string();
            numstat.author.name = captures.name("name").unwrap().as_str().to_string();
            numstat.author.email = captures.name("email").unwrap().as_str().to_string();
            continue;
        }

        // Parse date
        let date_re = regex::Regex::new(r"Date:\s+(?P<date>.*)").unwrap();
        if let Some(captures) = date_re.captures(line) {
            // TODO: there are a bug that the commit message contains `Date: `
            // To workaround, we will skip if the date is already parsed
            if numstat.date != default_date {
                continue;
            }

            let date = captures.name("date").unwrap().as_str().to_string();

            numstat.date = DateTime::parse_from_rfc2822(&date)
                .with_context(|| format!("Parsing block `{}`", block))
                .with_context(|| format!("Parsing date `{}`", date))?;
            continue;
        }

        // Merges
        let merge_re = regex::Regex::new(r"Merge:\s+(?P<merges>.*)").unwrap();
        if let Some(captures) = merge_re.captures(line) {
            let merges = captures.name("merges").unwrap().as_str();
            numstat.merges = merges.split(' ').map(|s| s.to_string()).collect();
            continue;
        }

        // Parse message
        // Message can be multiple lines
        if line.starts_with("    ") {
            numstat.message.push_str(line.trim());
            continue;
        }

        // Parse stats using regex
        // Each line contains two \t separated numbers and a path
        // 20       2       config/app_log/summary.ts
        let file_stat_re = regex::Regex::new(r"(\d+)\s+(\d+)\s+(.*)")?;

        if file_stat_re.is_match(line) {
            let captures = file_stat_re.captures(line).unwrap();

            let added = captures.get(1).unwrap().as_str().parse::<u32>()?;
            let deleted = captures.get(2).unwrap().as_str().parse::<u32>()?;
            let path = captures.get(3).unwrap().as_str().to_string();

            // Parse extension
            let extension = path.split('.').last().unwrap_or_default().to_string();
            // .github/workflows/{ci.yaml => rust-test.yaml}
            let extension = extension.to_lowercase().trim_end_matches('}').to_string();

            let stat = crate::numstat::Stat {
                added,
                deleted,
                path,
                extension,
            };

            numstat.stats.push(stat);
            continue;
        }
    }

    if numstat.commit.is_empty() {
        return Err(anyhow!("cannot parse this block commit: {}", block));
    }

    Ok(numstat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_numstat() {
        let raw = indoc! {"
            commit 1d4171694d9322ab22ee7cdd6712b83ecd8ae6c1 (HEAD -> master, origin/master, origin/HEAD)
            Merge: 32e30ab bbb7c8e
            Author: Duyet Le <5009534+duyet@users.noreply.github.com>
            Date:   Tue, 10 Jan 2023 00:35:39 +0700

                Merge pull request #42 from duyet/renovate/all-minor-patch

                chore(deps): update all non-major dependencies

            commit bbb7c8e78f07cd06dc015c7139cb174285cd6a8c (tag: v1.0.24+demo, feat)
            Author: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>
            Date:   Mon, 9 Jan 2023 14:24:18 +0000

                chore(deps): update all non-major dependencies

            3       3       Cargo.toml

            commit 32e30ab87f97dda21de104ba1454967d0d47cebb (tag: v1.0.23+demo, tag: v1.0.23)
            Merge: 0597f96 9209904
            Author: Duyet Le <5009534+duyet@users.noreply.github.com>
            Date:   Wed, 28 Dec 2022 16:17:38 +0700

                Merge pull request #41 from duyet/renovate/all-minor-patch

                fix(deps): update all non-major dependencies

            commit 4ba39f0bd2eb785bd8c5aa6f14fa2c8d357e92f7
            Author: Duyet Le <5009534+duyet@users.noreply.github.com>
            Date:   Sun, 14 Aug 2022 23:41:03 +0700

                chore: remove clippy from ci.yaml

            3       14      .github/workflows/rust-clippy.yml
            1       11      .github/workflows/{ci.yaml => rust-test.yaml}

            commit 920990462b055b8c9f556c2dced9181c93fd1674
            Author: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>
            Date:   Sun, 18 Dec 2022 23:10:25 +0000

                fix(deps): update all non-major dependencies

            3       3       Cargo.toml

            commit 4358eece77f51d54531fc52e5be92b46cc31aec9
            Author: Duyet Le <me@duyet.net>
            Date:   Wed, 10 Aug 2022 09:05:19 +0700

                feat: impl apply to Athena feature

            5       0       Cargo.toml
            128     4       src/apply.rs
            15      5       src/build.rs
            8       3       src/main.rs
            11      0       src/utils.rs

            commit 68a776b89c865a91a05047af6d8f2db2f03859d0
            Author: Duyet Le <me@duyet.net>
            Date:   Tue, 9 Aug 2022 20:56:49 +0700

                docs: update README.md

            53      5       README.md
            1       1       examples/base/table_1.sql
            1       0       examples/prd/index.sql
            1       4       tests/cli_build.rs
        "};

        let results = parse_from_str(raw).unwrap();
        println!("{:#?}", results);
        assert_eq!(results.len(), 7);

        let first = &results[0];
        assert_eq!(first.commit, "1d4171694d9322ab22ee7cdd6712b83ecd8ae6c1");
        assert_eq!(first.author.name, "Duyet Le");
        assert_eq!(first.author.email, "5009534+duyet@users.noreply.github.com");
        assert_eq!(first.date.to_rfc2822(), "Tue, 10 Jan 2023 00:35:39 +0700");
        assert_eq!(first.merges, vec!["32e30ab", "bbb7c8e"]);
        assert_eq!(first.tags.len(), 0);
        assert_eq!(first.branches, vec!["origin/master", "origin/HEAD"]);
        assert_eq!(first.message, "Merge pull request #42 from duyet/renovate/all-minor-patchchore(deps): update all non-major dependencies");
        assert_eq!(first.stats.len(), 0);

        // Parse tags and branches
        let second = &results[1];
        assert_eq!(second.commit, "bbb7c8e78f07cd06dc015c7139cb174285cd6a8c");
        assert_eq!(second.author.name, "renovate[bot]");
        assert_eq!(second.tags, vec!["v1.0.24+demo"]);
        assert_eq!(second.branches, vec!["feat"]);

        // Multiple tags
        let third = &results[2];
        assert_eq!(third.commit, "32e30ab87f97dda21de104ba1454967d0d47cebb");
        assert_eq!(third.tags, vec!["v1.0.23+demo", "v1.0.23"]);

        // Renaming files
        let fourth = &results[3];
        assert_eq!(fourth.commit, "4ba39f0bd2eb785bd8c5aa6f14fa2c8d357e92f7");
        assert_eq!(fourth.stats.len(), 2);
        assert_eq!(fourth.stats[0].added, 3);
        assert_eq!(fourth.stats[0].deleted, 14);
        assert_eq!(fourth.stats[0].path, ".github/workflows/rust-clippy.yml");
        assert_eq!(fourth.stats[1].added, 1);
        assert_eq!(fourth.stats[1].deleted, 11);
        assert_eq!(
            fourth.stats[1].path,
            ".github/workflows/{ci.yaml => rust-test.yaml}"
        );
        assert_eq!(fourth.stats[1].extension, "yaml");

        let last = &results[6];
        assert_eq!(last.commit, "68a776b89c865a91a05047af6d8f2db2f03859d0");
        assert_eq!(last.author.name, "Duyet Le");
        assert_eq!(last.author.email, "me@duyet.net");
        assert_eq!(last.date.to_rfc2822(), "Tue, 9 Aug 2022 20:56:49 +0700");
        assert_eq!(last.merges.len(), 0);
        assert_eq!(last.tags.len(), 0);
        assert_eq!(last.message, "docs: update README.md");
        assert_eq!(last.stats.len(), 4);
        assert_eq!(last.stats[0].added, 53);
        assert_eq!(last.stats[0].deleted, 5);
        assert_eq!(last.stats[0].path, "README.md");
        assert_eq!(last.stats[0].extension, "md");
        assert_eq!(last.stats[1].added, 1);
        assert_eq!(last.stats[1].deleted, 1);
        assert_eq!(last.stats[1].path, "examples/base/table_1.sql");
        assert_eq!(last.stats[1].extension, "sql");
        assert_eq!(last.stats[2].added, 1);
        assert_eq!(last.stats[2].deleted, 0);
        assert_eq!(last.stats[2].path, "examples/prd/index.sql");
        assert_eq!(last.stats[2].extension, "sql");
        assert_eq!(last.stats[3].added, 1);
        assert_eq!(last.stats[3].deleted, 4);
        assert_eq!(last.stats[3].path, "tests/cli_build.rs");
        assert_eq!(last.stats[3].extension, "rs");
    }
}
