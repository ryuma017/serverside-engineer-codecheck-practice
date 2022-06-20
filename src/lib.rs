use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const LIMIT: u32 = 10;
const OUTPUT_CSV_HEADER: &str = "rank,player_id,mean_score";

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
                                       // ^ 内部的には `io::BufReader` が使われるぽい(?)から多分早い

    let mut player_data_map = set_player_data_map(&mut reader)?;

    let mut mean_score_map = set_mean_score_map(&mut player_data_map)?;

    output_ranking(&mut mean_score_map)
}

fn set_player_data_map(reader: &mut csv::Reader<std::fs::File>) -> Result<HashMap<String, ScoreData>, anyhow::Error> {
    let mut player_data_map = HashMap::new();
    reader.deserialize().for_each(|result| {
        let record: LogValue = result.expect("Failed to deserialize a record");
        let score_data = player_data_map
            .entry(record.player_id)
            .or_insert_with(ScoreData::new);
        score_data.add(record.score);
    });
    Ok(player_data_map)
}

fn set_mean_score_map(player_data_map: &mut HashMap<String, ScoreData>) -> Result<BTreeMap<u64, Vec<String>>, anyhow::Error> {
    let mut mean_score_map = BTreeMap::new();
    player_data_map.iter().for_each(|(player_id, score_data)| {
        let mean_score = (score_data.sum as f64 / score_data.count as f64).round() as u64;
        let player_ids = mean_score_map.entry(mean_score).or_insert(vec![]);
        player_ids.push(player_id.clone());
    });
    Ok(mean_score_map)
}

fn output_ranking(mean_score_map: &mut BTreeMap<u64, Vec<String>>) -> Result<(), anyhow::Error> {
    let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(vec![]);
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

    print!("{}", String::from_utf8(writer.into_inner()?)?); // `into_inner` 内部で flush してる
    Ok(())
}