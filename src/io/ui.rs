use crate::errors::MoveError;
use crate::utils::types::Move;
use crate::utils::{ChessPosition, Position};
use std::io::{stdin, stdout, Write};

pub fn read_move() -> Result<Move, MoveError> {
    print!("Next move: ");
    stdout().flush()?;

    let mut next_move = String::new();
    stdin().read_line(&mut next_move)?;

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
