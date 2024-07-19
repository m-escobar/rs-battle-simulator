use std::collections::HashMap;
use std::fmt;
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


impl fmt::Display for PlayerActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}