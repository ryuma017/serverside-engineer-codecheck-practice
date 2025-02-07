use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use clap::Parser;

use serde::{Deserialize, Serialize};

const LIMIT: u32 = 10;
const OUTPUT_CSV_HEADER: &str = "rank,player_id,mean_score";

/// Reads CSV-formatted play logs of three specific columns and calculates the top 10 ranking players
#[derive(Parser)]
#[clap(about, long_about = None)]
pub struct Args {
    /// Path to CSV file
    #[clap(parse(from_os_str))]
    pub csv_file_path: PathBuf,
}

pub fn parse_args() -> Args {
    Args::parse()
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

pub fn run(file_path: PathBuf) -> Result<(), anyhow::Error> {
    let mut reader = csv::Reader::from_path(file_path)?;

    let mut player_data_map = HashMap::new();
    set_player_data_map(&mut reader, &mut player_data_map)?;

    let mut mean_score_map = BTreeMap::new();
    set_mean_score_map(&mut player_data_map, &mut mean_score_map)?;

    output_ranking(&mut mean_score_map)
}

fn set_player_data_map(
    reader: &mut csv::Reader<std::fs::File>,
    player_data_map: &mut HashMap<String, ScoreData>,
) -> Result<(), anyhow::Error> {
    reader.deserialize().for_each(|result| {
        let record: LogValue = result.expect("Failed to deserialize a record");
        let score_data = player_data_map
            .entry(record.player_id)
            .or_insert_with(ScoreData::new);
        score_data.add(record.score);
    });
    Ok(())
}

fn set_mean_score_map(
    player_data_map: &mut HashMap<String, ScoreData>,
    mean_score_map: &mut BTreeMap<u64, Vec<String>>,
) -> Result<(), anyhow::Error> {
    player_data_map.iter().for_each(|(player_id, score_data)| {
        let mean_score = (score_data.sum as f64 / score_data.count as f64).round() as u64;
        let player_ids = mean_score_map.entry(mean_score).or_insert(vec![]);
        player_ids.push(player_id.clone());
    });
    Ok(())
}

fn output_ranking(mean_score_map: &mut BTreeMap<u64, Vec<String>>) -> Result<(), anyhow::Error> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(vec![]);
    let mut count = 0;
    let mut rank = 1;
    println!("{OUTPUT_CSV_HEADER}");
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
