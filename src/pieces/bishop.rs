use crate::utils::Position;

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub fn can_move(origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    (src_line - dest_line).abs() == (src_col - dest_col).abs()
}
