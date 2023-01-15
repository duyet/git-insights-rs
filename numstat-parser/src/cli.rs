use clap::Parser;

/// Parse the git log --numstat output
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
#[clap(color(clap::ColorChoice::Auto))]
pub struct Cli {
    /// Path to the numstat.txt file
    /// Or path to the git repo
    pub path: std::path::PathBuf,
}

// Parse the command line arguments
pub fn parse() -> Cli {
    Cli::parse()
}
