use crate::utils::Position;
use crate::utils::types::Board;

use super::{bishop, rook, Piece};

pub const SYMBOLS: [char; 2] = ['\u{2655}', '\u{265B}'];

pub fn can_move(piece: Piece, destination: Position, board: Board) -> bool {
    bishop::can_move(piece, destination, board) || rook::can_move(piece, destination, board)
}
