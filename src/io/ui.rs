use crate::game_state::GameState;
use std::process::Command;

use crate::utils::constants;

pub fn print_board(game_state: GameState) {
    Command::new("clear")
        .status()
        .expect("Failed to clear screen");

    for (line, line_chess) in game_state.board().iter().zip(constants::LINES.iter()) {
        print!("{} ", line_chess);
        for opt_piece in line {
            match opt_piece {
                Some(piece) => print!("{} ", piece.symbol()),
                None => print!("{} ", constants::BLANK_SQUARE),
            }
        }
        println!();
    }

    print!("  ");

    for col_chess in constants::COLUMNS {
        print!("{} ", col_chess);
    }

    println!();
}
