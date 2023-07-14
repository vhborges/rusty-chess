use crate::utils::Position;

pub const SYMBOLS: [char; 2] = ['\u{2656}', '\u{265C}'];

pub fn can_move(origin: Position,  destination: Position) -> bool {
    let (line, col) = (origin.line, origin.col);

    // Logical XOR
    (line == destination.line) != (col == destination.col)
}
