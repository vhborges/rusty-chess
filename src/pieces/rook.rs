use crate::utils::{types::Board, Position};

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2656}', '\u{265C}'];

pub fn can_move(piece: Piece, destination: Position, board: Board) -> bool {
    let (line, col) = (piece.position.line, piece.position.col);

    // Logical XOR
    (line == destination.line) != (col == destination.col)
}
