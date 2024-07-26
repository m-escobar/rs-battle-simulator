use std::error::Error;
use std::io::{stdout, Stdout};
use rusty_audio::Audio;

use rs_battle_simulator::game_actions::*;
use rs_battle_simulator::game_lib::{restore_terminal, setup_terminal};
use rs_battle_simulator::game_ui::{game_over, print_header, print_players_grid};
use rs_battle_simulator::player::Player;
use rs_battle_simulator::players_parser::load_players;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let mut audio = Audio::new();

    audio.add("gameover", "audio/game-over.wav");

    setup_terminal(&mut stdout).expect("Error starting the Terminal");

    'mainloop: loop {
        play_game(&mut stdout)?;

        if play_again(&mut stdout) == 2 {
            break 'mainloop;
        }
    }

    audio.play("gameover");
    audio.wait();

    restore_terminal(&mut stdout).expect("Error restoring the Terminal");

    Ok(())
}

fn play_game(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    let loser: &str;
    let mut audio = Audio::new();

    audio.add("startup", "audio/start-game.wav");

    // out.queue(Clear(ClearType::All)).unwrap();
    // out.queue(MoveTo(0, 0)).unwrap();

    let players = match load_players() {
        Ok(players) => players,
        Err(_) => panic!("Error loading Players config."),
    };

    print_header(stdout).expect("TODO: panic message");

    audio.play("startup");
    
    let player_id = select_player(&players, stdout);
    let mut player: Player = players[player_id - 1].clone();

    let opponent_id = select_opponent(&players, &player_id);
    let mut opponent: Player = players[opponent_id - 1].clone();

    'gameloop: loop {
        print_players_grid(&player, &opponent, stdout)?;

        player.action = select_action(&player, stdout);
        opponent.action = select_opponent_action(&opponent);

        process_actions(&mut player, &mut opponent);


        if player.attributes["Health"] <= 0 {
            loser = "player";
            break 'gameloop;
        }

        if opponent.attributes["Health"] <= 0 {
            loser = "opponent";
            break 'gameloop;
        }
    }

    if loser == "player" || loser == "opponent" {
        let _ = game_over(loser, stdout);
    }

    Ok(())
}
