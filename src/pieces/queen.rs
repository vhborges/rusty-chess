use crate::utils::Position;

use super::{bishop, rook};

pub const SYMBOLS: [char; 2] = ['\u{2655}', '\u{265B}'];

pub fn can_move(origin: Position,  destination: Position) -> bool {
    bishop::can_move(origin, destination) || rook::can_move(origin, destination)
}
