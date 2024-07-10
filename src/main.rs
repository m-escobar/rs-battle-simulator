use std::error::Error;
use std::io::{stdout, Write};

use rs_battle_simulator::game_ui::print_header;
use rs_battle_simulator::setup_terminal;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    setup_terminal(&mut stdout).expect("Error starting the Terminal");

    print_header(&mut stdout).expect("TODO: panic message");

    stdout.flush()?;


    // out.queue(Clear(ClearType::All)).unwrap();
    // out.queue(MoveTo(0, 0)).unwrap();
    // out.flush().unwrap();


    // restore_terminal(&mut stdout).expect("Error restoring the Terminal");
    
    Ok(())
}
