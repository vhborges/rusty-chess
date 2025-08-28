use crate::Board;
use crate::movement::Position;

use super::{Rook, bishop};

pub const SYMBOLS: [char; 2] = ['\u{2655}', '\u{265B}'];

pub fn can_move(board: &Board, origin: Position, destination: Position) -> bool {
    bishop::can_move(board, origin, destination) || Rook::can_move(board, origin, destination)
}

pub fn attacks(board: &Board, origin: Position, destination: Position) -> bool {
    bishop::attacks(board, origin, destination) || Rook::attacks(board, origin, destination)
}

pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
    let mut v1 = bishop::get_possible_moves(board, source);
    v1.append(&mut Rook::get_possible_moves(board, source));

    v1
}
