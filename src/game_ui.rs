use std::error::Error;
use std::io::Stdout;

use crossterm::{cursor, QueueableCommand};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use unicode_segmentation::UnicodeSegmentation;

use crate::player::Player;

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

pub fn print_players_grid(player: &Player, opponent: &Player, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    let space :String = String::from(" ");

    print_header(stdout).expect("TODO: panic message");

    print_line(&player.name, &opponent.name, 0, 0, stdout)?;
    print_line(&player.description, &opponent.description, 0, 0, stdout)?;
    print_line(&space, &space, 0, 0, stdout)?;
    
    let mut sorted: Vec<_> = player.attributes.iter().collect();
    sorted.sort_by_key(|a| a.0);
    
    // Print attributes block
    for attribute in sorted {
        let key = &attribute.0;
        let spaces = add_spaces(key.len(), 10);

        let player_grid = get_progress_bar(*attribute.1);
        let opponent_grid = get_progress_bar(opponent.attributes[attribute.0]);

        let player_line = key.to_string() + &spaces + &player_grid;
        let opponent_line = key.to_string() + &spaces + &opponent_grid;

        print_line(&player_line , &opponent_line, 1, 0, stdout)?;

        if attribute.0 == "Health" {
            print_line(&space, &space, 0, 0, stdout)?;
        }
    }

    print_line(&space, &space, 0, 0, stdout)?;

    stdout.queue(Print("=======================================================================\x0d\x0a"))?;

    Ok(())
}

fn print_line(param1: &String, param2: &String, margin: u8, fix_size: usize, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    let mut spaces:String = "".to_string();

    if margin != 0 {
        for _i in 1..=margin {
            spaces += "    ";
        }
    }

    let block1 = format!("| {}{}", spaces, param1);
    let block2 = format!("| {}{}", spaces, param2);

    let result_block1:String = add_spaces(block1.graphemes(true).count() - fix_size, 34);
    let result_block2:String = add_spaces(block2.graphemes(true).count() - fix_size, 34);

    let formated_line = format!("{}{}{}{}|\x0d\x0a", block1, result_block1, block2, result_block2);

    stdout.queue(Print(formated_line))?;

    Ok(())
}

fn add_spaces(str_size: usize, block_length: usize) -> String {
    let mut result:String = "".to_string();

    for _pos in str_size..=block_length {
        result += " ";
    }

    result
}

fn get_progress_bar(value: i32) -> String {
    let fullchar = "▮";
    let emptychar = "▯";
    let mut result: String = "".to_string();

    for pos in 1..=10 {
        if pos <= value {
            result += fullchar;
        } else {
            result += emptychar;
        }
    }

    result
}
