use std::error::Error;
use std::io::Stdout;

use crossterm::cursor::{Hide, Show};
use crossterm::ExecutableCommand;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

pub fn setup_terminal(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    // terminal::enable_raw_mode()?;

    stdout
        .execute(EnterAlternateScreen)?
        .execute(Hide)?;
    // .execute(SetForegroundColor(Color::DarkGrey))?
    // .execute(SetBackgroundColor(Color::Black))?;

    Ok(())
}


pub fn restore_terminal(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    stdout
        .execute(LeaveAlternateScreen)?
        .execute(Show)?;
    // .execute(ResetColor)?;

    // terminal::disable_raw_mode()?;

    Ok(())
}
