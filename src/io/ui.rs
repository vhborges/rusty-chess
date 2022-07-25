use crate::errors::MoveError;
use crate::game_state::GameState;
use crate::utils::{ChessPosition, Position, constants};
use std::io::{stdin, stdout, Write};
use std::process::Command;

pub fn print_board(game_state: &GameState) {
    Command::new("clear")
        .status()
        .expect("Failed to clear screen");

    for (line, line_chess) in game_state.board().iter().zip(constants::LINES.iter()) {
        print!("{} ", line_chess);
        for opt_piece in line {
            match opt_piece {
                Some(piece) => print!("{} ", piece),
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

    for piece in game_state.captured_white_pieces() {
        print!("{} ", piece)
    }

    println!();

    for piece in game_state.captured_black_pieces() {
        print!("{} ", piece)
    }

    println!()
}

pub fn read_move() -> Result<(Position, Position), MoveError> {
    print!("Next move: ");
    stdout().flush().expect("Unable to flush screen.");

    let mut next_move = String::new();
    stdin().read_line(&mut next_move).expect("Unable to read move.");

    let moves: Vec<&str> = next_move.split(' ').collect();
    if moves.len() != 2 {
        return Err(MoveError::InvalidMove);
    }

    let (source, dest) = (moves[0].trim(), moves[1].trim());
    if source.len() != 2 {
        return Err(MoveError::InvalidSquare(source.to_string()));
    }
    if dest.len() != 2 {
        return Err(MoveError::InvalidSquare(dest.to_string()));
    }

    let source_pos: Position = ChessPosition::new(
        source.chars().nth(1).unwrap(),
        source.chars().nth(0).unwrap(),
    )
    .try_into()?;

    let dest_pos: Position = ChessPosition::new(
        dest.chars().nth(1).unwrap(),
        dest.chars().nth(0).unwrap()
    )
    .try_into()?;

    Ok((source_pos, dest_pos))
}
