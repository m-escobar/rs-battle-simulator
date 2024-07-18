use std::error::Error;
use std::io::{stdout};
use rs_battle_simulator::players_parser::load_players;
use rs_battle_simulator::game_actions::select_player;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    let players = match load_players() {
        Ok(players) => players,
        Err(_) => panic!("Error loading Players config."),
    };

    // setup_terminal(&mut stdout).expect("Error starting the Terminal");
    
    let _selected_player = select_player(&players, &mut stdout);

    // stdout.flush()?;


    // out.queue(Clear(ClearType::All)).unwrap();
    // out.queue(MoveTo(0, 0)).unwrap();
    // out.flush().unwrap();


    // restore_terminal(&mut stdout).expect("Error restoring the Terminal");
    
    Ok(())
}
