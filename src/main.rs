mod cli;

fn main() -> Result<(), anyhow::Error> {
    let arg = cli::parse_args();

    cli::run(arg.csv_file_path)?;

    Ok(())
}
