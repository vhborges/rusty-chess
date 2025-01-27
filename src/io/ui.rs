use crate::GameState;
use crate::types::Position;
use crate::utils::constants::{BLANK_SQUARE, BOARD_SIZE, COLUMNS, LINES};
use std::io::{self, Write, stdin, stdout};

pub fn read_move() -> io::Result<String> {
    print!("Next move: ");
    stdout().flush()?;

    let mut next_move = String::new();
    stdin().read_line(&mut next_move)?;

    Ok(next_move.trim().to_owned())
}

pub fn print_game(game_state: &GameState) {
    clearscreen::clear().expect("Failed to clear screen");

    for (line, line_chess) in (0..BOARD_SIZE).zip(LINES.iter()) {
        print!("{} ", line_chess);
        for column in 0..BOARD_SIZE {
            let maybe_piece = game_state.get_piece(Position::new(line, column));
            match maybe_piece {
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

    for piece in game_state.captured_white_pieces() {
        print!("{} ", piece)
    }

    println!();

    for piece in game_state.captured_black_pieces() {
        println!("{} ", piece)
    }
}
