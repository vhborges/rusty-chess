use crate::utils::Position;

pub const SYMBOLS: [char; 2] = ['\u{2654}', '\u{265A}'];

pub fn can_move(origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    let horizontal_distance = dest_col - src_col;
    let vertical_distance = dest_line - src_line;

    if (-1..=1).contains(&horizontal_distance) && (-1..=1).contains(&vertical_distance) {
        return true;
    }

    false
}

pub fn attacks(origin: Position, destination: Position) -> bool {
    can_move(origin, destination)
}
