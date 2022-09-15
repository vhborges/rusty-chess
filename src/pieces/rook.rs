use crate::utils::{types::Board, Position};

pub const SYMBOLS: [char; 2] = ['\u{2656}', '\u{265C}'];

pub fn can_move(origin: Position, destination: Position, board: Board) -> bool {
    let (line, col) = (*origin.line(), *origin.col());

    // Logical XOR
    (line == *destination.line()) != (col == *destination.col())
}
