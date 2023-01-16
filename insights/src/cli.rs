use clap::Parser;

/// Parse the output of `git log --numstat --date=rfc`
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
#[clap(color(clap::ColorChoice::Auto))]
pub struct Cli {
    /// Path to the numstat.txt file
    /// or path to local/remote the git repo
    #[arg(required = true)]
    pub path: Vec<std::path::PathBuf>,
    /// Filtered by year. e.g. --year 2022 --year 2023
    #[arg(short, long)]
    pub year: Vec<u32>,
    /// Filtered by author(s)
    #[arg(short, long)]
    pub author: Vec<String>,
    /// Filtered by ignore author(s)
    #[arg(long)]
    pub ignore_author: Vec<String>,
    /// Filter out by extensions
    #[arg(short, long)]
    pub ignore_ext: Vec<String>,
    /// Remap the author email. e.g. --remap-email "me@duyet.net<=5009534+duyet@users.noreply.github.com,lvduit08@gmail.com"
    #[arg(long)]
    pub remap_email: Vec<String>,
    /// Remap the author name. e.g. --remap-name "Duyet Le=>Duyet"
    #[arg(long)]
    pub remap_name: Vec<String>,
    /// Remap the extension. e.g. --remap-ext "tsx=>ts"
    #[arg(long)]
    pub remap_ext: Vec<String>,
}

// Parse the command line arguments
pub fn parse() -> Cli {
    Cli::parse()
}
