use std::error::Error;
use std::io::stdout;

use rs_battle_simulator::game_actions::{select_action, select_opponent, select_player};
use rs_battle_simulator::player::Player;
use rs_battle_simulator::players_parser::load_players;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let mut player_action: usize;
    
    let players = match load_players() {
        Ok(players) => players,
        Err(_) => panic!("Error loading Players config."),
    };

    // setup_terminal(&mut stdout).expect("Error starting the Terminal");

    let player_id = select_player(&players, &mut stdout);
    let player: Player = players[player_id - 1].clone();
    
    let opponent_id = select_opponent(&players, &player_id);
    let opponent: Player = players[opponent_id - 1].clone();
    
    
    // loop {
        player_action = select_action(&player,  &mut stdout);    
    
    
    // }
    
    
    
    // out.queue(Clear(ClearType::All)).unwrap();
    // out.queue(MoveTo(0, 0)).unwrap();
    // out.flush().unwrap();


    // restore_terminal(&mut stdout).expect("Error restoring the Terminal");

    Ok(())
}
