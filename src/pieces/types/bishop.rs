use crate::Board;
use crate::movement::{Position, PositionI8};

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

// TODO unit test this function with a source in the edges of the board
pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
    let mut result = Vec::new();

    for dx in (-1..=1).step_by(2) {
        for dy in (-1..=1).step_by(2) {
            let mut dest =
                match PositionI8::new(source.line as i8 + dx, source.col as i8 + dy).try_into() {
                    Ok(pos) => pos,
                    Err(_) => continue,
                };

            while !board.is_position_occupied(dest) {
                result.push(dest);
                dest = match PositionI8::new(dest.line as i8 + dx, dest.col as i8 + dy).try_into() {
                    Ok(pos) => pos,
                    Err(_) => break,
                };
            }
        }
    }

    result
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
