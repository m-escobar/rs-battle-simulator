use std::error::Error;
use std::io::Stdout;
use crossterm::{cursor, QueueableCommand};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

pub fn print_header(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> { 
    stdout
        .queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .queue(Print("=======================================================================\x0d\x0a"))?
        .queue(Print("\x0d\x0a"))?
        .queue(Print("                    ⚔️   Battle Simulator   ⚔️                         \x0d\x0a"))?
        .queue(Print("\x0d\x0a"))?
        .queue(Print("\x0d\x0a"))?
        .queue(Print("                        |\x0d\x0a"))?
        .queue(Print("            ////////////|---------------------------------,\x0d\x0a"))?
        .queue(Print("            `^^^^^^^^^^^|--------------------------------\"\x0d\x0a"))?
        .queue(Print("\x0d\x0a"))?
        .queue(Print("\x0d\x0a"))?
        .queue(Print("=======================================================================\x0d\x0a"))?;
    
    Ok(())
}
