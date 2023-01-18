# Generate commit insights

[![codecov](https://codecov.io/gh/duyet/git-insights-rs/branch/master/graph/badge.svg?token=VCDqi5hfza)](https://codecov.io/gh/duyet/git-insights-rs)
[![cargo-clippy](https://github.com/duyet/git-insights-rs/actions/workflows/cargo-clippy.yaml/badge.svg)](https://github.com/duyet/git-insights-rs/actions/workflows/cargo-clippy.yaml)
[![cargo-test](https://github.com/duyet/git-insights-rs/actions/workflows/cargo-test.yaml/badge.svg)](https://github.com/duyet/athena-rs/actions/workflows/cargo-test.yaml)
[![Code coverage](https://github.com/duyet/git-insights-rs/actions/workflows/cov.yaml/badge.svg)](https://github.com/duyet/athena-rs/actions/workflows/cov.yaml)
[![cargo-fmt](https://github.com/duyet/git-insights-rs/actions/workflows/cargo-fmt.yaml/badge.svg)](https://github.com/duyet/athena-rs/actions/workflows/cargo-fmt.yaml)

Generating commit insights from local.


# Installation

<!-- BEGIN INSTALLATION -->
```bash
$ cargo install --git https://github.com/duyet/git-insights-rs
$ insights --help

Parse the output of `git log --numstat --date=rfc`

Usage: girs [OPTIONS] <PATH>...

Arguments:
  <PATH>...  Path to the numstat.txt file or path to local/remote the git repositories

Options:
  -y, --year <YEAR>                    Only including these years. e.g. --year 2022 --year 2023
  -a, --author <AUTHOR>                Only including these author(s)
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
insights <folder contains multiple git dir>
insights <git dir>
insights <git dir> --year=2023
insights <git dir> --author="Duyet Le" --author="Duet"
insights <git dir> --remap-email="me@duyet.net<=5009534+duyet@users.noreply.github.com" --author="Duet"
insights <git dir> --remap-ext="js,jsx=>js" --remap-ext "ts<=tsx,tss"
insights <git dir> --ignore-ext=gitignore
```

## Example

<!-- BEGIN DEMO -->
```bash
$ git clone https://github.com/duyet/git-insights-rs /tmp/git-insights-rs
$ insights /tmp/git-insights-rs
```

Output:

```
Data summary: shape: (1, 7)
┌──────────────┬──────────────┬──────────────────┬─────────────────┬───────┬─────────┬─────────────┐
│ author_count ┆ commit_count ┆ authors          ┆ extensions      ┆ added ┆ deleted ┆ last commit │
╞══════════════╪══════════════╪══════════════════╪═════════════════╪═══════╪═════════╪═════════════╡
│ 2            ┆ 37           ┆ ["Duyet Le",     ┆ ["toml", "md",  ┆ 2465  ┆ 668     ┆ 2023-01-18  │
│              ┆              ┆ "Duyet Le", ...  ┆ ...             ┆       ┆         ┆ 06:51:03    │
│              ┆              ┆ "Du...           ┆ "gitignore"]    ┆       ┆         ┆             │
└──────────────┴──────────────┴──────────────────┴─────────────────┴───────┴─────────┴─────────────┘

Commit by authors: shape: (2, 2)
┌─────────────┬────────┐
│ author_name ┆ commit │
╞═════════════╪════════╡
│ Duyet Le    ┆ 26     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ duyetbot    ┆ 11     │
└─────────────┴────────┘

Commit by author by date: shape: (2, 3)
┌─────────────┬────────────┬────────┐
│ author_name ┆ year_month ┆ commit │
╞═════════════╪════════════╪════════╡
│ Duyet Le    ┆ 2023-01    ┆ 26     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ duyetbot    ┆ 2023-01    ┆ 11     │
└─────────────┴────────────┴────────┘

Total commit by months: shape: (1, 2)
┌────────────┬────────┐
│ year_month ┆ commit │
╞════════════╪════════╡
│ 2023-01    ┆ 37     │
└────────────┴────────┘

Commit by author: shape: (2, 2)
┌─────────────┬────────┐
│ author_name ┆ commit │
╞═════════════╪════════╡
│ Duyet Le    ┆ 26     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ duyetbot    ┆ 11     │
└─────────────┴────────┘

Top languages by commit: shape: (5, 2)
┌──────────┬────────┐
│ language ┆ commit │
╞══════════╪════════╡
│ md       ┆ 18     │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ yaml     ┆ 13     │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ rs       ┆ 12     │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ toml     ┆ 7      │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ sh       ┆ 3      │
└──────────┴────────┘

Top commit by day of week: shape: (4, 3)
┌─────┬─────────────┬────────┐
│ n   ┆ day_of_week ┆ commit │
╞═════╪═════════════╪════════╡
│ 1   ┆ Monday      ┆ 12     │
├╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ 2   ┆ Tuesday     ┆ 8      │
├╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ 3   ┆ Wednesday   ┆ 5      │
├╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ 7   ┆ Sunday      ┆ 12     │
└─────┴─────────────┴────────┘

```
<!-- END DEMO -->

# License

MIT.
