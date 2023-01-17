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

Usage: insights [OPTIONS] <PATH>...

Arguments:
  <PATH>...  Path to the numstat.txt file or path to local/remote the git repo

Options:
  -y, --year <YEAR>                    Filtered by year. e.g. --year 2022 --year 2023
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
Data summary: shape: (5, 10)
┌────────┬────────┬─────────┬───────────┬────────────┬──────┬─────────┬─────────┬───────┬──────────┐
│ descri ┆ commit ┆ date    ┆ author_na ┆ author_ema ┆ path ┆ extensi ┆ added   ┆ delet ┆ year_mon │
│ be     ┆        ┆         ┆ me        ┆ il         ┆      ┆ on      ┆         ┆ ed    ┆ th       │
╞════════╪════════╪═════════╪═══════════╪════════════╪══════╪═════════╪═════════╪═══════╪══════════╡
│ count  ┆ 26.0   ┆ 26.0    ┆ 26.0      ┆ 26.0       ┆ 26.0 ┆ 26.0    ┆ 26.0    ┆ 26.0  ┆ 26.0     │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ mean   ┆ null   ┆ null    ┆ null      ┆ null       ┆ null ┆ null    ┆ 31.2692 ┆ 1.461 ┆ null     │
│        ┆        ┆         ┆           ┆            ┆      ┆         ┆ 31      ┆ 538   ┆          │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ std    ┆ null   ┆ null    ┆ null      ┆ null       ┆ null ┆ null    ┆ 60.0450 ┆ 3.744 ┆ null     │
│        ┆        ┆         ┆           ┆            ┆      ┆         ┆ 22      ┆ 124   ┆          │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ min    ┆ null   ┆ 1.6738e ┆ null      ┆ null       ┆ null ┆ null    ┆ 0.0     ┆ 0.0   ┆ null     │
│        ┆        ┆ 15      ┆           ┆            ┆      ┆         ┆         ┆       ┆          │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ max    ┆ null   ┆ 1.6738e ┆ null      ┆ null       ┆ null ┆ null    ┆ 315.0   ┆ 17.0  ┆ null     │
│        ┆        ┆ 15      ┆           ┆            ┆      ┆         ┆         ┆       ┆          │
└────────┴────────┴─────────┴───────────┴────────────┴──────┴─────────┴─────────┴───────┴──────────┘

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
│ toml      ┆ 2      │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ rs        ┆ 2      │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ gitignore ┆ 1      │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ txt       ┆ 1      │
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
