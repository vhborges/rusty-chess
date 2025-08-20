use crate::GameState;
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

    game_state.board().print_board();

    println!();

    for piece in game_state.captured_white_pieces() {
        print!("{piece} ")
    }

    println!();

    for piece in game_state.captured_black_pieces() {
        print!("{piece} ")
    }

    println!();
}
