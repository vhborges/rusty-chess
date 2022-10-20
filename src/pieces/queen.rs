use crate::utils::Position;

use super::{bishop, rook, Piece};

pub const SYMBOLS: [char; 2] = ['\u{2655}', '\u{265B}'];

pub fn can_move(piece: Piece, destination: Position) -> bool {
    bishop::can_move(piece, destination) || rook::can_move(piece, destination)
}
