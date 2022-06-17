use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use clap::Parser;
use serde::{Deserialize, Serialize};

const LIMIT: u32 = 10;

#[derive(Parser)]
#[clap(about, long_about = None)]
struct Args {
    /// Path to CSV file
    csv_file_path: String,
}

#[derive(Deserialize)]
struct LogValue {
    // create_timestamp 使わない
    player_id: String,
    score: u64,
}


fn main() -> anyhow::Result<()> {
    let arg = Args::parse();
    println!("{}", arg.csv_file_path);
    Ok(())
}