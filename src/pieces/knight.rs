use crate::utils::Position;
use crate::utils::types::Board;

pub const SYMBOLS: [char; 2] = ['\u{2658}', '\u{265E}'];

pub fn can_move(origin: Position, destination: Position, board: Board) -> bool {
    let (line, col) = (*origin.line(), *origin.col());

    let vertical_distance = (line as i8 - *destination.line() as i8).abs();
    let horizontal_distance = (col as i8 - *destination.col() as i8).abs();

    (vertical_distance == 1 && horizontal_distance == 2) ||
    (vertical_distance == 2 && horizontal_distance == 1)
}
