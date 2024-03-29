use crate::utils::Position;
use crate::utils::types::Board;

use super::{bishop, rook};

pub const SYMBOLS: [char; 2] = ['\u{2655}', '\u{265B}'];

pub fn can_move(board: &Board, origin: Position, destination: Position) -> bool {
    bishop::can_move(board, origin, destination) || rook::can_move(board, origin, destination)
}

pub fn attacks(board: &Board, origin: Position, destination: Position) -> bool {
    can_move(board, origin, destination)
}
