use polars::prelude::*;

use crate::cli::Cli;

const DEFAULT_REMAP_EXT: [&str; 4] = ["tsx=>ts", "jsx=>js", "htm=>html", "yml=>yaml"];
const DEFAULT_IGNORE_EXT: [&str; 4] = ["lock", "staging", "local", "license"];

pub fn preprocess(df: DataFrame, args: &Cli) -> LazyFrame {
    let df = df
        .lazy()
        .with_column(col("date").dt().strftime("%Y-%m").alias("year_month"));

    // Drop duplicates
    let df = df.unique_stable(None, UniqueKeepStrategy::Last);

    // Filter by year
    let df = if !args.year.is_empty() {
        df.filter(
            col("date")
                .dt()
                .year()
                .is_in(lit(Series::from_iter(args.year.clone()))),
        )
    } else {
        df
    };

    // Filter by authors
    let df = if !args.author.is_empty() {
        df.filter(col("author_name").is_in(lit(Series::from_iter(args.author.clone()))))
    } else {
        df
    };

    // Filter by ignore authors
    let df = if !args.ignore_author.is_empty() {
        df.filter(
            col("author_name")
                .is_in(lit(Series::from_iter(args.ignore_author.clone())))
                .not(),
        )
    } else {
        df
    };

    // Normalize extensions
    let df = df
        .with_column(col("extension").str().to_lowercase().alias("extension"))
        .filter(
            col("extension")
                .is_in(lit(Series::from_iter(DEFAULT_IGNORE_EXT)))
                .not(),
        );

    // Remap extensions using default
    // Convert array to slice
    let exts = &DEFAULT_REMAP_EXT
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let df = modify_column(df, "extension", exts);

    // Ignore extensions
    let df = if !args.ignore_ext.is_empty() {
        let exts = args.ignore_ext.clone();

        df.filter(col("extension").is_in(lit(Series::from_iter(exts))).not())
    } else {
        df
    };

    // Remap the author name
    let df = if !args.remap_name.is_empty() {
        modify_column(df, "author_name", &args.remap_name)
    } else {
        df
    };

    // Remap the author email
    let df = if !args.remap_email.is_empty() {
        modify_column(df, "author_email", &args.remap_email)
    } else {
        df
    };

    // Remap the language (extension)

    let df = if !args.remap_ext.is_empty() {
        modify_column(df, "extension", &args.remap_ext.clone())
    } else {
        df
    };

    // Should cache the preprocessed to prevent reprocessing
    df.cache()
}

fn modify_column(df: LazyFrame, col_name: &str, from_to: &[String]) -> LazyFrame {
    // Replace the value of the author_name column
    // Replace the value [from]=>[to] or [to]<=[from]
    let remap = from_to
        .iter()
        .flat_map(|r| {
            let (froms, to) = if r.contains("<=") {
                let mut split = r.split("<=");
                let to = split.next().unwrap();
                let froms = split.next().unwrap();
                (froms, to)
            } else if r.contains("=>") {
                let mut split = r.split("=>");
                let froms = split.next().unwrap();
                let to = split.next().unwrap();
                (froms, to)
            } else {
                panic!("Invalid remap format: {}", r);
            };

            let froms = froms.split(',').collect::<Vec<_>>();
            froms
                .iter()
                .map(|f| (f.to_string(), to.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<(String, String)>>();

    let mut df = df;

    // Modify the column in place
    for (from, to) in remap {
        df = df.with_column(
            when(col(col_name).str().contains(lit(from), false))
                .then(lit(to))
                .otherwise(col(col_name))
                .alias(col_name),
        );
    }

    df
}
