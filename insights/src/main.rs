mod cli;

use anyhow::{Context, Result};
use numstat_parser::{parse_from_path, Numstat};
use polars::frame::row::Row;
use polars::prelude::*;
use rayon::prelude::*;

const DEFAULT_REMAP_EXT: [&str; 4] = ["tsx=>ts", "jsx=>js", "htm=>html", "yml=>yaml"];
const DEFAULT_IGNORE_EXT: [&str; 4] = ["lock", "staging", "local", "license"];

fn main() -> Result<()> {
    env_logger::init();
    let args = cli::parse();

    let result: Vec<Numstat> = parse_from_path(&args.path)
        .with_context(|| format!("Parsing from {}", args.path.display()))?;

    // Map result to Vec<Row>
    let rows = result
        .par_iter()
        .flat_map(|n| {
            n.stats
                .par_iter()
                .map(|f| {
                    Row::new(vec![
                        AnyValue::Utf8(&n.commit),
                        AnyValue::Datetime(
                            n.date.timestamp_micros(),
                            TimeUnit::Microseconds,
                            &None,
                        ),
                        AnyValue::Utf8(&n.author.name),
                        AnyValue::Utf8(&n.author.email),
                        AnyValue::Utf8(&f.path),
                        AnyValue::Utf8(&f.extension),
                        AnyValue::UInt32(f.added),
                        AnyValue::UInt32(f.deleted),
                    ])
                })
                .collect::<Vec<Row>>()
        })
        .collect::<Vec<Row>>();

    let mut df = DataFrame::from_rows(&rows)?;

    // Change column names
    df.rename("column_0", "commit")?
        .rename("column_1", "date")?
        .rename("column_2", "author_name")?
        .rename("column_3", "author_email")?
        .rename("column_4", "path")?
        .rename("column_5", "extension")?
        .rename("column_6", "added")?
        .rename("column_7", "deleted")?;

    // Print the DataFrame
    log::debug!("{}\n", preprocess(df.clone(), &args).collect()?);

    // Query: How many lines of code were added per author?
    println!(
        "Commit by authors in 2022: {}\n",
        preprocess(df.clone(), &args)
            .groupby([col("author_name")])
            .agg([col("commit").count().alias("commit_count")])
            .sort_by_exprs(
                &[col("author_name"), col("commit_count")],
                [false, false],
                false
            )
            .collect()?
    );

    // Query: Commit by author by date, convert date to YYYY-MM
    println!(
        "Commit by author by date in 2022: {}\n",
        preprocess(df.clone(), &args)
            .groupby([col("author_name"), col("year_month")])
            .agg([col("commit").count()])
            .sort_by_exprs(&[col("author_name"), col("commit")], [false, true], false)
            .collect()?
    );

    println!(
        "Total commit by months: {}\n",
        preprocess(df.clone(), &args)
            .groupby([col("year_month")])
            .agg([col("commit").count()])
            .sort_by_exprs(&[col("year_month")], [false], false)
            .collect()?
    );

    println!(
        "Commit by author: {}\n",
        preprocess(df.clone(), &args)
            .groupby([col("author_name")])
            .agg([col("commit").count()])
            .sort_by_exprs(&[col("commit")], [true], false)
            .collect()?
    );

    println!(
        "Top languages by commit: {}\n",
        preprocess(df.clone(), &args)
            .groupby([col("extension").alias("language")])
            .agg([col("commit").count()])
            .sort_by_exprs(&[col("commit")], [true], false)
            .limit(5)
            .collect()?
    );

    println!(
        "Top commit by day of week: {}\n",
        preprocess(df.clone(), &args)
            .with_column(col("date").dt().weekday().alias("n"))
            .with_column(col("date").dt().strftime("%A").alias("day_of_week"))
            .groupby([col("n"), col("day_of_week")])
            .agg([col("commit").count()])
            .sort_by_exprs(&[col("n")], [false], false)
            .collect()?
    );

    Ok(())
}

fn preprocess(df: DataFrame, args: &cli::Cli) -> LazyFrame {
    let df = df
        .lazy()
        .with_column(col("date").dt().strftime("%Y-%m").alias("year_month"));

    // Filter by year
    let df = if let Some(year) = args.year {
        df.filter(col("date").dt().year().eq(year))
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
                .is_in(lit(Series::from_iter(args.author.clone())))
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

    if !args.remap_ext.is_empty() {
        modify_column(df, "extension", &args.remap_ext.clone())
    } else {
        df
    }
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
            when(col(col_name).str().contains(from))
                .then(lit(to))
                .otherwise(col(col_name))
                .alias(col_name),
        );
    }

    df
}
