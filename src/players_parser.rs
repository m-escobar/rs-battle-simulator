use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::{Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub description: String,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub power: i32,
    pub actions: Vec<PlayerActions>,
    pub items: HashMap<String, i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PlayerActions {
    SwordAttack,
    DrinkPotion,
    Dodge,
    FireAttack,
    TailHit,
    FootHit,
    HandHit,
}


pub fn load_players() -> Result<Vec<Player>> {
    // Read JSON file
    let file_path = "data/players.json".to_owned();
    let json_data = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    let players: Vec<Player> = serde_json::from_str(&json_data)?;

    Ok(players)
}
