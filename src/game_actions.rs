use std::collections::HashMap;
use std::io;
use std::io::{Stdout, Write};

use crossterm::QueueableCommand;
use crossterm::style::Print;
use rand::Rng;
use rusty_audio::Audio;

use crate::player::Player;
use crate::player::PlayerActions::{Dodge, DrinkPotion, SwordAttack};

pub fn select_opponent(players: &[Player], player_id: &usize) -> usize{
    let mut selected_opponent: usize;

    loop {
        selected_opponent = rand::thread_rng().gen_range(1..=players.len());

        if selected_opponent != *player_id {
            break;
        }
    }

    selected_opponent
}

pub fn select_player(players: &[Player],  stdout: &mut Stdout) -> usize {
    let message: &str = "Choose your Player and go to the BATTLE field!";
    let mut player_list: HashMap<i32, String> = HashMap::new();

    players
        .iter()
        .for_each(|p| {
            player_list.insert(p.id, p.name.clone());
        });

    select_option(player_list, message, stdout)
}

pub fn select_action(player: &Player,  stdout: &mut Stdout) -> usize {
    let message: &str = "Your turn! Choose what you will do:";
    let mut actions= HashMap::<i32, String>::new();
    let mut audio = Audio::new();
    let mut counter:usize = 1;
    let mut idx = HashMap::<usize, i32>::new();
    
    audio.add("sword", "audio/sword.wav");

    player.actions
        .iter()
        .clone()
        .enumerate()
        .for_each(|(i, p)| {

            if *p != DrinkPotion || player.items["potion"] > 0 {
                actions.insert(counter as i32, p.description());
                idx.insert(counter, i as i32);
                counter += 1;
            }
        });
    
    let option = select_option(actions, message, stdout);

    if player.actions[option - 1] == SwordAttack {
        audio.play("sword");
        audio.wait();
    }
    
    idx[&option] as usize
}

pub fn select_option(options: HashMap<i32, String>, message: &str, stdout: &mut Stdout) -> usize {
    let mut selected_option: usize;
    let mut user_input = String::new();
    let counter = options.len();
    let formated_message = String::from("\x0d\x0a") + message + "\x0d\x0a\x0d\x0a";

    loop {
        let _ = stdout.queue(Print(&formated_message));

        let mut items: Vec<_> = options.iter().collect();
        items.sort_by(|x,y| x.0.cmp(y.0));

        items
            .iter()
            .for_each(|item| {
                let _ = stdout.queue(Print(item.0.to_string() + " - " + item.1 + "\x0d\x0a"));
            });

        let _ = stdout.queue(Print("\x0d\x0a"));
        let _ = stdout.flush();

        io::stdin().read_line(&mut user_input).expect("Error: unable to read user input.");

        selected_option = user_input.trim().parse().unwrap_or(0);

        if selected_option > 0 && selected_option <= counter {
            break;
        } else {
            user_input = "".to_string();
        }
    }

    selected_option
}

pub fn select_opponent_action(opponent: &Player) -> usize{
    rand::thread_rng().gen_range(1..=opponent.actions.len()) - 1
}

pub fn process_actions(player: &mut Player, opponent: &mut Player) {
    let mut audio = Audio::new();
    audio.add("potion", "audio/drink-potion.wav");

    let mut dodge = false;

    if player.actions[player.action] == Dodge {
        *player.attributes.get_mut("Health").unwrap() -= 1;
        dodge = true;
    }

    if opponent.actions[opponent.action] == Dodge {
        *opponent.attributes.get_mut("Health").unwrap() -= 1;
        dodge = true;
    }

    if !dodge {
        let player_power = ((player.attributes["Attack"] * player.attributes["Power"]) /
                                (player.attributes["Defense"] * player.attributes["Speed"])) as f64 * 0.9;

        let opponent_power = ((opponent.attributes["Attack"] * opponent.attributes["Power"]) /
                                  (opponent.attributes["Defense"] * opponent.attributes["Speed"])) as f64 * 0.7;

        if player_power >= opponent_power {
            *opponent.attributes.get_mut("Health").unwrap() -= 1;
        } else if player_power != opponent_power {
            *player.attributes.get_mut("Health").unwrap() -= 1;
        };
    }

    if player.actions[player.action] == DrinkPotion && player.items["potion"] > 0 {
        *player.attributes.get_mut("Health").unwrap() = 10;
        *player.items.get_mut("potion").unwrap() -= 1;
        audio.play("potion");
        audio.wait();
    }

    if opponent.actions[opponent.action] == DrinkPotion && opponent.items["potion"] > 0 {
        *opponent.attributes.get_mut("Health").unwrap() = 10;
        *opponent.items.get_mut("potion").unwrap() -= 1;
    }

    player.action = 0;
    opponent.action = 0;
}

pub fn play_again(stdout: &mut Stdout) -> usize {
    let message: &str = "Do you want to play again?";
    let options: HashMap<i32, String> = HashMap::from([
                                                (1, String::from("Yes")),
                                                (2, String::from("No"))
                                        ]);

    select_option(options, message, stdout)
}
