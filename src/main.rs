use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use clap::Parser;
use serde::{Deserialize, Serialize};

const LIMIT: u32 = 10;

/// Reads CSV-formatted play logs of three specific columns and calculates the top 10 ranking players
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

fn main() -> anyhow::Result<()> {
    let arg = Args::parse();
    let path = Path::new(&arg.csv_file_path);
    let mut reader = csv::Reader::from_path(path)?;

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
        let player_id_vec = mean_score_map.entry(mean_score).or_insert(vec![]);
        player_id_vec.push(player_id.clone());
    });

    let mut writer = csv::Writer::from_writer(vec![]);
    let mut count = 1;
    let mut rank = 1;
    for (mean_score, player_id_vec) in mean_score_map.iter().rev() {
        for player_id in player_id_vec {
            writer.serialize(RankingValue {
                rank,
                player_id,
                mean_score: *mean_score,
            })?;
            count += 1;
        }
        rank += player_id_vec.len() as u64;

        if count == LIMIT {
            break;
        }
    }

    print!("{}", String::from_utf8(writer.into_inner()?)?);

    Ok(())
}
