use crate::Board;
use crate::movement::{Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2658}', '\u{265E}'];

pub fn can_move(source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let vertical_distance = (src.line - dest.line).abs();
    let horizontal_distance = (src.col - dest.col).abs();

    (vertical_distance == 1 && horizontal_distance == 2)
        || (vertical_distance == 2 && horizontal_distance == 1)
}

pub fn attacks(origin: Position, destination: Position) -> bool {
    can_move(origin, destination)
}

pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
    let mut result = Vec::new();

    for dx in (-2..=2).step_by(4) {
        for dy in (-1..=1).step_by(2) {
            let dest_i8 = PositionI8::new(source.line as i8 + dy, source.col as i8 + dx);
            if let Ok(dest) = dest_i8.try_into()
                && !board.is_position_occupied(dest)
            {
                result.push(dest);
            }
        }
    }

    for dx in (-1..=1).step_by(2) {
        for dy in (-2..=2).step_by(4) {
            let dest_i8 = PositionI8::new(source.line as i8 + dy, source.col as i8 + dx);
            if let Ok(dest) = dest_i8.try_into()
                && !board.is_position_occupied(dest)
            {
                result.push(dest);
            }
        }
    }

    result
}
