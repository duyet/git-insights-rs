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
```
<!-- END DEMO -->

# License

MIT.
