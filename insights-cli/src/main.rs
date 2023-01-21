mod cli;
mod preprocess;

use anyhow::{bail, Context, Result};
use numstat_parser::{parse_from_path, Numstat};
use polars::frame::row::Row;
use polars::prelude::*;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::env;

use crate::preprocess::preprocess;

fn main() -> Result<()> {
    env::set_var("POLARS_FMT_TABLE_HIDE_COLUMN_DATA_TYPES", "1");
    env::set_var("POLARS_FMT_MAX_ROWS", "20");
    env::set_var("POLARS_FMT_MAX_COLS", "10");

    env_logger::init();
    let args = cli::parse();

    let result: Vec<Numstat> =
        parse_from_path(&args.path).with_context(|| format!("Parsing from {:?}", args.path))?;

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

    let mut heading = HashMap::new();
    let mut query: BTreeMap<&str, DataFrame> = BTreeMap::new();

    macro_rules! query {
        ($name:literal, $title:literal, $expr:expr) => {
            heading.insert($name, $title);
            query.insert($name, $expr);
        };
    }

    // Query: data summary.
    query!(
        "summary",
        "Summary",
        preprocess(df.clone(), &args)
            .select([
                col("author_name").n_unique().alias("author_count"),
                col("commit").n_unique().alias("commit_count"),
                col("author_name").list().alias("authors"),
                col("extension").list().alias("extensions"),
                col("added").sum(),
                col("deleted").sum(),
                col("date").max().alias("last commit"),
            ])
            .collect()?
    );

    // Query: How many lines of code were added per author?
    query!(
        "commit_by_author",
        "Commit by author",
        preprocess(df.clone(), &args)
            .groupby([col("author_name")])
            .agg([col("commit").n_unique()])
            .sort_by_exprs(&[col("commit")], [true], false)
            .collect()?
    );

    // Query: Total commits by month
    query!(
        "commit_by_month",
        "Commit by month",
        preprocess(df.clone(), &args)
            .groupby([col("year_month")])
            .agg([col("commit").n_unique()])
            .sort_by_exprs(&[col("year_month")], [false], false)
            .collect()?
    );

    // Query: Commit by author by date, convert date to YYYY-MM
    query!(
        "commit_by_author_by_month",
        "Commit by author by month",
        preprocess(df.clone(), &args)
            .groupby([col("author_name"), col("year_month")])
            .agg([col("commit").n_unique()])
            .sort_by_exprs(&[col("author_name"), col("commit")], [false, true], false)
            .collect()?
    );

    // Query: Top languages
    query!(
        "top_languages",
        "Top languages",
        preprocess(df.clone(), &args)
            .groupby([col("extension").alias("language")])
            .agg([col("commit").n_unique()])
            .sort_by_exprs(&[col("commit")], [true], false)
            .limit(5)
            .collect()?
    );

    // Query: Top commit by weekday
    query!(
        "commit_by_weekday",
        "Commit by weekday",
        preprocess(df.clone(), &args)
            .with_column(col("date").dt().weekday().alias("n"))
            .with_column(col("date").dt().strftime("%A").alias("weekday"))
            .groupby([col("n"), col("weekday")])
            .agg([col("commit").n_unique()])
            .sort_by_exprs(&[col("n")], [false], false)
            .collect()?
    );

    match args.output {
        cli::Output::None => {
            for (k, v) in query {
                println!("{}: {}\n", heading.get(&k).unwrap_or(&k), v)
            }
        }
        cli::Output::Json => {
            use serde_json::{json, to_value, Value};

            let values = to_value(&query)?;
            let mut out = json!({});

            for (k, v) in values.as_object().unwrap().iter() {
                let mut cols = json!({});

                for col in v["columns"].as_array().unwrap().iter() {
                    let key = col["name"].as_str().unwrap();

                    let values = match col["values"].as_array().unwrap() {
                        values if values[0].is_object() => values
                            .iter()
                            .map(|v| v["values"].clone())
                            .collect::<Vec<Value>>(),
                        values => values.to_vec(),
                    };

                    cols[key] = Value::Array(values);
                }

                out[k] = serde_json::to_value(cols)?;
            }

            println!("{:#}", out);
        }
        _ => {
            bail!("Not implemented yet.");
        }
    }

    Ok(())
}
