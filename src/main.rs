use std::path::PathBuf;

use clap::Parser;

/// Reads CSV-formatted play logs of three specific columns and calculates the top 10 ranking players
#[derive(Parser)]
#[clap(about, long_about = None)]
struct Args {
    /// Path to CSV file
    #[clap(parse(from_os_str))]
    csv_file_path: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let arg = Args::parse();
    get_ranking::run(arg.csv_file_path)?;
    Ok(())
}
