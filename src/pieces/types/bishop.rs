use crate::Board;
use crate::movement::{Direction, Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub fn can_move(board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let nr_of_squares = (dest.col - src.col).unsigned_abs() as usize;

    is_move_valid(src, dest) && board.is_path_clear(src, dest, nr_of_squares)
}

pub fn attacks(board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let nr_of_squares = ((dest.col - src.col).abs() - 1) as usize;

    is_move_valid(src, dest) && board.is_path_clear(src, dest, nr_of_squares)
}

// TODO unit test this function with a source in the edges of the board
pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
    let mut result = Vec::new();
    let pos_i8 = source.into();

    for dx in (-1..=1).step_by(2) {
        for dy in (-1..=1).step_by(2) {
            let direction = Direction::new(dx, dy, pos_i8);

            for pos in direction {
                if board.is_position_occupied(pos) {
                    break;
                }
                result.push(pos);
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
