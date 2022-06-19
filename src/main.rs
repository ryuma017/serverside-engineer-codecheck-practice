use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

const LIMIT: u32 = 10;

/// Reads CSV-formatted play logs of three specific columns and calculates the top 10 ranking players
#[derive(Parser)]
#[clap(about, long_about = None)]
struct Args {
    /// Path to CSV file
    #[clap(parse(from_os_str))]
    csv_file_path: PathBuf,
}

#[derive(Deserialize)]
struct LogValue {
    // create_timestamp 使わない
    player_id: String,
    score: u64,
}

#[derive(Debug, Serialize)]
struct RankingValue<'a> {
    rank: u64,
    player_id: &'a str,
    mean_score: u64,
}

struct ScoreData {
    sum: u64,
    count: u64,
}

impl ScoreData {
    fn new() -> Self {
        Self { sum: 0, count: 0 }
    }

    fn add(&mut self, score: u64) {
        self.sum += score;
        self.count += 1;
    }
}

fn main() -> Result<(), anyhow::Error> {
    let arg = Args::parse();
    // let reader = BufReader::new(File::open(arg.csv_file_path)?);
    // let mut reader = csv::Reader::from_reader(reader);
    let mut reader = csv::Reader::from_path(arg.csv_file_path)?;

    let mut player_data_map = HashMap::new();
    reader.deserialize().for_each(|result| {
        let record: LogValue = result.expect("Failed to deserialize a record");
        let score_data = player_data_map
            .entry(record.player_id)
            .or_insert_with(ScoreData::new);
        score_data.add(record.score);
    });

    let mut mean_score_map = BTreeMap::new();
    player_data_map.iter().for_each(|(player_id, score_data)| {
        let mean_score = (score_data.sum as f64 / score_data.count as f64).round() as u64;
        let player_ids = mean_score_map.entry(mean_score).or_insert(vec![]);
        player_ids.push(player_id);
    });

    // let mut out = BufWriter::new(stdout().lock());
    let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(vec![]);
    let mut count = 0;
    let mut rank = 1;
    println!("rank,player_id,mean_score");
    for (mean_score, player_ids) in mean_score_map.iter_mut().rev() {
        player_ids.sort();
        for player_id in player_ids.iter() {
            writer.serialize(RankingValue {
                rank,
                player_id: player_id.as_str(),
                mean_score: *mean_score,
            })?;
            count += 1;
        }
        rank += player_ids.len() as u64;

        if count >= LIMIT {
            break;
        }
    }

    print!("{}", String::from_utf8(writer.into_inner()?)?);
    Ok(())
}
