mod cli;
mod git;
mod parse;

use anyhow::{bail, Result};
use log::debug;
use rayon::prelude::*;

fn main() -> Result<()> {
    env_logger::init();
    let args = cli::parse();

    match args.path {
        path if path.is_dir() => {
            let dirs = if path.join(".git").exists() {
                vec![git::get_log(&path)?]
            } else {
                let git_dirs = std::fs::read_dir(path)?
                    .filter_map(|entry| {
                        let path = entry.expect("could not parse path").path();
                        if path.join(".git").exists() {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                git_dirs
                    .par_iter()
                    .map(|entry| {
                        debug!("Running git log on {}", entry.display());
                        git::get_log(entry).unwrap()
                    })
                    .collect()
            };

            debug!("Scanned {} git dirs", dirs.len());

            if dirs.is_empty() {
                bail!("No .git found");
            }

            let gitlogs = dirs.join("\n");
            debug!("Number of gitlog lines: {}", gitlogs.lines().count());

            let numstats = parse::numstat(&gitlogs)?;
            println!("{:#?}", numstats);
        }
        path if path.is_file() => {
            // Parse the file, if the argument is a path to a numstat.txt file
            let output = std::fs::read_to_string(&path).unwrap();
            let numstats = parse::numstat(&output)?;
            println!("{:#?}", numstats);
        }
        _ => {
            bail!("Invalid path");
        }
    }

    Ok(())
}
