use crate::types::{Position, PositionI8};

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
