use crate::utils::types::Board;
use crate::utils::Position;

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

pub fn can_move(origin: Position, destination: Position, board: Board) -> bool {
    let (line, col) = (*origin.line(), *origin.col());

    let (dest_line, dest_col) = (*destination.line(), *destination.col());

    dest_col == col && (dest_line as i8 - line as i8).abs() == 1
}
