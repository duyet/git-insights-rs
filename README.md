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
│ 1            ┆ 3            ┆ ["Duyet Le",     ┆ ["toml",        ┆ 813   ┆ 38      ┆ 2023-01-15  │
│              ┆              ┆ "Duyet Le", ...  ┆ "toml", ...     ┆       ┆         ┆ 11:10:04    │
│              ┆              ┆ "Du...           ┆ "gitignore"...  ┆       ┆         ┆             │
└──────────────┴──────────────┴──────────────────┴─────────────────┴───────┴─────────┴─────────────┘

Commit by authors: shape: (1, 2)
┌─────────────┬────────┐
│ author_name ┆ commit │
╞═════════════╪════════╡
│ Duyet Le    ┆ 3      │
└─────────────┴────────┘

Commit by author by date: shape: (1, 3)
┌─────────────┬────────────┬────────┐
│ author_name ┆ year_month ┆ commit │
╞═════════════╪════════════╪════════╡
│ Duyet Le    ┆ 2023-01    ┆ 3      │
└─────────────┴────────────┴────────┘

Total commit by months: shape: (1, 2)
┌────────────┬────────┐
│ year_month ┆ commit │
╞════════════╪════════╡
│ 2023-01    ┆ 3      │
└────────────┴────────┘

Commit by author: shape: (1, 2)
┌─────────────┬────────┐
│ author_name ┆ commit │
╞═════════════╪════════╡
│ Duyet Le    ┆ 3      │
└─────────────┴────────┘

Top languages by commit: shape: (5, 2)
┌───────────┬────────┐
│ language  ┆ commit │
╞═══════════╪════════╡
│ rs        ┆ 2      │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ toml      ┆ 2      │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ txt       ┆ 1      │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ gitignore ┆ 1      │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ yaml      ┆ 1      │
└───────────┴────────┘

Top commit by day of week: shape: (1, 3)
┌─────┬─────────────┬────────┐
│ n   ┆ day_of_week ┆ commit │
╞═════╪═════════════╪════════╡
│ 7   ┆ Sunday      ┆ 3      │
└─────┴─────────────┴────────┘

```
<!-- END DEMO -->

# License

MIT.
