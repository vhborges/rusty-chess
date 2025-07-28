use crate::types::{Board, Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub fn can_move(board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let nr_of_squares = (dest.col - src.col).abs();

    is_move_valid(src, dest) && board.is_path_clear(src, dest, nr_of_squares)
}

pub fn attacks(board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let nr_of_squares = (dest.col - src.col).abs() - 1;

    is_move_valid(src, dest) && board.is_path_clear(src, dest, nr_of_squares)
}

fn is_move_valid(source: PositionI8, destination: PositionI8) -> bool {
    if (source.line == destination.line) || (source.col == destination.col) {
        return false;
    }

    if (source.line - destination.line).abs() != (source.col - destination.col).abs() {
        return false;
    }

    true
}
