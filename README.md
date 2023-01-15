# Generate commit insights

[![codecov](https://codecov.io/gh/duyet/git-insights-rs/branch/master/graph/badge.svg?token=VCDqi5hfza)](https://codecov.io/gh/duyet/git-insights-rs)
[![cargo-clippy](https://github.com/duyet/git-insights-rs/actions/workflows/cargo-clippy.yml/badge.svg)](https://github.com/duyet/athena-rs/actions/workflows/cargo-clippy.yml)
[![cargo-test](https://github.com/duyet/git-insights-rs/actions/workflows/cargo-test.yaml/badge.svg)](https://github.com/duyet/athena-rs/actions/workflows/cargo-test.yaml)
[![Code coverage](https://github.com/duyet/git-insights-rs/actions/workflows/cov.yaml/badge.svg)](https://github.com/duyet/athena-rs/actions/workflows/cov.yaml)
[![cargo-doc](https://github.com/duyet/git-insights-rs/actions/workflows/cargo-doc.yaml/badge.svg)](https://github.com/duyet/athena-rs/actions/workflows/cargo-doc.yaml)
[![cargo-fmt](https://github.com/duyet/git-insights-rs/actions/workflows/cargo-fmt.yaml/badge.svg)](https://github.com/duyet/athena-rs/actions/workflows/cargo-fmt.yaml)

Generating commit insights from local.


# Installation

<!-- BEGIN INSTALLATION -->
```bash
$ cargo install --git https://github.com/duyet/git-insights-rs
$ insights --help

Parse the git log --numstat output

Usage: insights [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to the numstat.txt file Or path to the git repo

Options:
  -y, --year <YEAR>                    Filtered by year
  -a, --author <AUTHOR>                Filtered by author(s)
      --ignore-author <IGNORE_AUTHOR>  Filtered by ignore author(s)
  -i, --ignore-ext <IGNORE_EXT>        Filter out by extensions
      --remap-email <REMAP_EMAIL>      Remap the author email. e.g. --remap-email "me@duyet.net<=5009534+duyet@users.noreply.github.com,lvduit08@gmail.com"
      --remap-name <REMAP_NAME>        Remap the author name. e.g. --remap-name "Duyet Le=>Duyet"
      --remap-ext <REMAP_EXT>          Remap the extension. e.g. --remap-ext "tsx=>ts"
  -h, --help                           Print help
  -V, --version                        Print version
```
<!-- END INSTALLATION -->

# Usages

```bash
insights <git dir>
```

For example:

<!-- BEGIN DEMO -->
```bash
$ git clone $demo_repo $demo_repo_dir
$ insights $demo_repo_dir
```

Output:

```
Commit by authors in 2022: shape: (1, 2)
┌─────────────┬──────────────┐
│ author_name ┆ commit_count │
│ ---         ┆ ---          │
│ str         ┆ u32          │
╞═════════════╪══════════════╡
│ Duyet Le    ┆ 27           │
└─────────────┴──────────────┘

Commit by author by date in 2022: shape: (1, 3)
┌─────────────┬────────────┬────────┐
│ author_name ┆ year_month ┆ commit │
│ ---         ┆ ---        ┆ ---    │
│ str         ┆ str        ┆ u32    │
╞═════════════╪════════════╪════════╡
│ Duyet Le    ┆ 2023-01    ┆ 27     │
└─────────────┴────────────┴────────┘

Total commit by months: shape: (1, 2)
┌────────────┬────────┐
│ year_month ┆ commit │
│ ---        ┆ ---    │
│ str        ┆ u32    │
╞════════════╪════════╡
│ 2023-01    ┆ 27     │
└────────────┴────────┘

Commit by author: shape: (1, 2)
┌─────────────┬────────┐
│ author_name ┆ commit │
│ ---         ┆ ---    │
│ str         ┆ u32    │
╞═════════════╪════════╡
│ Duyet Le    ┆ 27     │
└─────────────┴────────┘

Top languages by commit: shape: (5, 2)
┌──────────┬────────┐
│ language ┆ commit │
│ ---      ┆ ---    │
│ str      ┆ u32    │
╞══════════╪════════╡
│ rs       ┆ 14     │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ toml     ┆ 6      │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ yaml     ┆ 4      │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ license  ┆ 1      │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ txt      ┆ 1      │
└──────────┴────────┘

Top commit by day of week: shape: (1, 3)
┌─────┬─────────────┬────────┐
│ n   ┆ day_of_week ┆ commit │
│ --- ┆ ---         ┆ ---    │
│ u32 ┆ str         ┆ u32    │
╞═════╪═════════════╪════════╡
│ 7   ┆ Sunday      ┆ 27     │
└─────┴─────────────┴────────┘

```
<!-- END DEMO -->

# License

MIT.
