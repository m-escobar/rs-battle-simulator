use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: i32,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlayerActions {
    SwordAttack,
    DrinkPotion,
    Dodge,
    FireAttack,
    TailHit,
    FootHit,
    HandHit,
}
