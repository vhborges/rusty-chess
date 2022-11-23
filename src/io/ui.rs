use std::process::Command;

use crate::game_state::GameState;
use crate::utils::constants::{BLANK_SQUARE, COLUMNS, LINES};
use std::io::{self, stdin, stdout, Write};

pub fn read_move() -> io::Result<String> {
    print!("Next move: ");
    stdout().flush()?;

    let mut next_move = String::new();
    stdin().read_line(&mut next_move)?;

    Ok(next_move.trim().to_owned())
}

pub fn print_game(game_state: &GameState) {
    Command::new("clear")
        .status()
        .expect("Failed to clear screen");

    for (line, line_chess) in game_state.board.iter().zip(LINES.iter()) {
        print!("{} ", line_chess);
        for opt_piece in line {
            match opt_piece {
                Some(piece) => print!("{} ", piece),
                None => print!("{} ", BLANK_SQUARE),
            }
        }
        println!();
    }

    print!("  ");

    for col_chess in COLUMNS {
        print!("{} ", col_chess);
    }

    println!();

    for piece in &game_state.captured_white_pieces {
        print!("{} ", piece)
    }

    println!();

    for piece in &game_state.captured_black_pieces {
        print!("{} ", piece)
    }

    println!()
}
