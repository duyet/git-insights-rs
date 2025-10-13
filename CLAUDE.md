# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`git-insights-rs` is a Rust CLI tool that generates commit insights from Git repositories. It parses git log data and produces analytical reports about commit patterns, author contributions, language usage, and temporal trends using Polars for data processing.

## Workspace Structure

This is a Cargo workspace with two crates:
- **`numstat-parser/`**: Library crate that parses `git log --numstat` output into structured data
- **`insights-cli/`**: Binary crate that processes parsed data with Polars and generates reports

## Development Commands

### Build and Run
```bash
# Build entire workspace
cargo build

# Build with optimizations
cargo build --release

# Run the CLI (requires git repository path)
cargo run -- <path-to-git-repo>

# Run with arguments
cargo run -- /path/to/repo --year 2023 --author "Name"

# Install locally
cargo install --path insights-cli
```

### Testing
```bash
# Run all tests in workspace
cargo test

# Run tests for specific crate
cargo test -p numstat_parser
cargo test -p girs

# Run specific test file
cargo test --test parse_dir
cargo test --test remap

# Run with logging output
RUST_LOG=debug cargo test
```

### Linting and Formatting
```bash
# Format all code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Run clippy
cargo clippy

# Run clippy with all features
cargo clippy --all-features --all-targets
```

### Code Coverage
```bash
# Generate coverage (requires tarpaulin)
cargo tarpaulin --out Html --output-dir coverage
```

## Architecture

### Data Flow Pipeline

1. **Input Parsing** (`numstat-parser` crate):
   - `parse_from_path()` → entry point that handles paths to git repos or directories
   - `git.rs` → executes `git log --numstat --date=rfc` commands
   - `parse_from_str()` → parses raw git output into `Numstat` structs

2. **Data Transformation** (`insights-cli` crate):
   - `main.rs` → converts `Vec<Numstat>` into Polars DataFrame rows
   - `preprocess.rs` → applies filters (year, author, extensions) and remapping logic
   - Polars LazyFrame operations for efficient aggregations

3. **Query and Output**:
   - Multiple pre-defined queries in `main.rs` (commits by author, by month, top languages, etc.)
   - Output formats: human-readable tables (default) or JSON (`--output json`)

### Key Data Structures

**`Numstat`** (numstat-parser/src/numstat.rs):
```rust
pub struct Numstat {
    pub commit: String,
    pub author: Author,        // name, email
    pub date: DateTime<FixedOffset>,
    pub stats: Vec<Stat>,      // added/deleted lines per file
    // metadata: merges, branches, tags, message
}
```

**DataFrame Schema** (insights-cli/src/main.rs:54-61):
- `commit`, `date`, `author_name`, `author_email`, `path`, `extension`, `added`, `deleted`

### Important Implementation Details

**Preprocessing Pipeline** (preprocess.rs):
- Default extension remapping: `tsx=>ts`, `jsx=>js`, `htm=>html`, `yml=>yaml`
- Default ignored extensions: `lock`, `staging`, `local`, `license`
- Remap syntax supports both `from=>to` and `to<=from1,from2`
- All operations use Polars LazyFrame for lazy evaluation and caching

**Parallel Processing**:
- Uses Rayon for parallel iteration when converting Numstat to DataFrame rows
- Each commit's stats are processed in parallel via `par_iter()`

**Git Command** (numstat-parser/src/git.rs):
- Executes: `git log --numstat --date=rfc --all`
- Expects RFC 2822 date format parsing

## Testing Strategy

Tests are located in `insights-cli/tests/` and use:
- `assert_cmd` for CLI integration tests
- `tempfile` for creating temporary test repos
- `predicates` for output assertions

Test files:
- `parse_dir.rs` - parsing git directories
- `parse_multiple.rs` - handling multiple repos
- `remap.rs` - extension/author remapping
- `parse_remote.rs` - remote repo handling
- `output_json_html.rs` - output format validation

## CI/CD

GitHub Actions workflows:
- `cargo-test.yaml` - run test suite
- `cargo-clippy.yaml` - linting
- `cargo-fmt.yaml` - format check
- `cov.yaml` - code coverage with Codecov

## Common Patterns

### Adding New Query
1. Add query definition in `main.rs` using the `query!` macro
2. Use `preprocess(df.clone(), &args)` as starting point
3. Apply Polars transformations (group_by, agg, sort_by_exprs)
4. Add entry to heading HashMap for display name

### Adding CLI Option
1. Add field to `Cli` struct in `cli.rs`
2. Implement filtering/remapping logic in `preprocess()` function
3. Follow existing patterns: filters use `.filter()`, remaps use `modify_column()`

### Extending Parser
- Modify `parse_from_str.rs` to handle new git log formats
- Update `Numstat` struct if adding new metadata fields
- Ensure thread-safety for parallel processing with Rayon

## Dependencies

Key dependencies and their usage:
- **polars** (0.51.0) - DataFrame operations, SQL-like queries, lazy evaluation
- **rayon** (1.10.0) - parallel iteration for performance
- **clap** (4.5.4) - CLI argument parsing with derive macros
- **chrono** (0.4.38) - datetime parsing and manipulation
- **regex** (1.10.4) - pattern matching in git output
- **anyhow** (1.0.82) - error handling with context
