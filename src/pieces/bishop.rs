use crate::utils::Position;
use crate::utils::types::Board;

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub fn can_move(origin: Position, destination: Position, board: Board) -> bool {
    let (line, col) = (*origin.line(), *origin.col());

    (line as i8 - *destination.line() as i8).abs() == (col as i8 - *destination.col() as i8).abs()
}
