use crate::utils::Position;
use crate::utils::types::Board;

use super::{bishop, rook};

pub const SYMBOLS: [char; 2] = ['\u{2655}', '\u{265B}'];

pub fn can_move(origin: Position, destination: Position, board: Board) -> bool {
    bishop::can_move(origin, destination, board) || rook::can_move(origin, destination, board)
}
