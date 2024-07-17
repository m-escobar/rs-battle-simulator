use std::fs;
use serde_json::{Result};
use crate::player::Player;

pub fn load_players() -> Result<Vec<Player>> {
    // Read JSON file
    let file_path = "data/players.json".to_owned();
    let json_data = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    let players: Vec<Player> = serde_json::from_str(&json_data)?;

    Ok(players)
}
