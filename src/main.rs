use std::error::Error;
use std::io::{stdout, Write};

use crossterm::{cursor, QueueableCommand};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use rs_battle_simulator::{restore_terminal, setup_terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    setup_terminal(&mut stdout).expect("Error starting the Terminal");


    stdout
        .queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(10, 10))?
        .queue(Print("Hello, world!\x0d\x0a"))?;

    stdout.flush()?;


    // out.queue(Clear(ClearType::All)).unwrap();
    // out.queue(MoveTo(0, 0)).unwrap();
    // out.flush().unwrap();


    // restore_terminal(&mut stdout).expect("Error restoring the Terminal");
    
    Ok(())
}
