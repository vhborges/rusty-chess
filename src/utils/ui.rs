use crate::game_state::GameState;
use std::process::Command;

use super::constants::board;

pub fn print_board(game_state: GameState) {
    Command::new("clear")
        .status()
        .expect("Failed to clear screen");

    for (line, line_chess) in game_state.board().iter().zip(board::LINES.iter()) {
        print!("{} ", line_chess);
        for opt_piece in line {
            match opt_piece {
                Some(piece) => print!("{} ", piece.symbol()),
                None => print!("{} ", board::BLANK_SQUARE),
            }
        }
        println!();
    }

    print!("  ");

    for col_chess in board::COLUMNS {
        print!("{} ", col_chess);
    }

    println!();
}
