mod cli;
mod git;
mod parse;

fn main() {
    let args = cli::parse();

    match args.path {
        path if path.is_dir() => {
            // Run git log --numstat, if the argument is a path to a git repo
            // Parse the output
            if let Ok(output) = git::log_numstat(&path) {
                let numstats = parse::numstat(&output);
                println!("{:#?}", numstats);
            }
        }
        path if path.is_file() => {
            // Parse the file, if the argument is a path to a numstat.txt file
            let output = std::fs::read_to_string(&path).unwrap();
            let numstats = parse::numstat(&output);
            println!("{:#?}", numstats);
        }
        _ => {
            panic!("Invalid path");
        }
    }
}
