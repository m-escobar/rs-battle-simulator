use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayersAttributes {
    Health, 
    Attack,
    Defense,
    Speed,
    Power
}

impl PlayerActions {
    pub fn description(&self) -> String {
        match *self {
            PlayerActions::SwordAttack => String::from("Sword Attack"),
            PlayerActions::DrinkPotion => String::from("Drink Potion"),
            PlayerActions::Dodge => String::from("Dodge"),
            PlayerActions::FireAttack => String::from("Fire Attack"),
            PlayerActions::TailHit => String::from("Tail Hit"),
            PlayerActions::FootHit => String::from("Foot Hit"),
            PlayerActions::HandHit => String::from("Hand Hit"),
        }
    }

    pub fn requirements(&self) -> Option<String> {
        match *self {
            PlayerActions::SwordAttack => Some(String::from("Sword")),
            PlayerActions::DrinkPotion => Some(String::from("Potion")),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub attributes: HashMap<String, i32>,
    pub actions: Vec<PlayerActions>,
    pub items: HashMap<String, i32>,
    pub action: usize,
}
