use std::error::Error;
use std::io::stdout;

use rs_battle_simulator::game_actions::*;
use rs_battle_simulator::game_ui::{print_header, print_players_grid};
use rs_battle_simulator::player::Player;
use rs_battle_simulator::players_parser::load_players;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    
    let players = match load_players() {
        Ok(players) => players,
        Err(_) => panic!("Error loading Players config."),
    };

    // setup_terminal(&mut stdout).expect("Error starting the Terminal");
    print_header(&mut stdout).expect("TODO: panic message");

    let player_id = select_player(&players, &mut stdout);
    let mut player: Player = players[player_id - 1].clone();
    
    let opponent_id = select_opponent(&players, &player_id);
    let mut opponent: Player = players[opponent_id - 1].clone();
    
    // loop {
        print_players_grid(&player, &opponent, &mut stdout)?;
    
        player.action = select_action(&player,  &mut stdout);
        opponent.action = select_opponent_action(&opponent);

        process_actions(&mut player, &mut opponent);

    // }
    
    
    
    // out.queue(Clear(ClearType::All)).unwrap();
    // out.queue(MoveTo(0, 0)).unwrap();
    // out.flush().unwrap();


    // restore_terminal(&mut stdout).expect("Error restoring the Terminal");

    Ok(())
}
