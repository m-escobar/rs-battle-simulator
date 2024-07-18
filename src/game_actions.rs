use std::io;
use std::io::{Stdout, Write};

use crossterm::QueueableCommand;
use crossterm::style::Print;
use rand::Rng;
use crate::game_ui::print_header;
use crate::player::Player;

pub fn select_player(players: &[Player], stdout: &mut Stdout) -> usize {
    let mut user_input = String::new();
    let mut selected_payer: usize;
    let player_counter = players.len();
    let mut name: String = Default::default();

    loop {
        print_header(stdout).expect("TODO: panic message");

        stdout.queue(Print("\x0d\x0aChoose your Player and go to the BATTLE field!\x0d\x0a\x0d\x0a")).expect("TODO: panic message");

        players.iter().clone()
            .for_each(|p| {
                name = format!("{} - {}\x0d\x0a", p.id, p.name);
                let _ = stdout.queue(Print(name.to_string()));

            });

        let _ = stdout.queue(Print("\x0d\x0a"));

        let _ = stdout.flush();

        io::stdin().read_line(&mut user_input).expect("Error: unable to read user input.");

        selected_payer = user_input.trim().parse().unwrap_or(0);

        if selected_payer > 0 && selected_payer <= player_counter {
            break;
        } else {
            user_input= "".to_string();
        }
    }

    selected_payer
}

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

// function game_actions.select_opponent(players_list, player_id)
// local selected_opponent = 0
//
// repeat
// local opponent = math.random(#players_list)
//
// if opponent ~= player_id
// then selected_opponent = opponent
// end
//
// until selected_opponent > 0
//
// return selected_opponent
// end
