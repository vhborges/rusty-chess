use crate::types::Position;

pub const SYMBOLS: [char; 2] = ['\u{2658}', '\u{265E}'];

pub fn can_move(origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    let vertical_distance = (src_line - dest_line).abs();
    let horizontal_distance = (src_col - dest_col).abs();

    (vertical_distance == 1 && horizontal_distance == 2)
        || (vertical_distance == 2 && horizontal_distance == 1)
}

pub fn attacks(origin: Position, destination: Position) -> bool {
    can_move(origin, destination)
}
