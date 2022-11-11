use crate::utils::Position;

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2658}', '\u{265E}'];

pub fn can_move(piece: Piece, destination: Position) -> bool {
    let (line, col) = (piece.position.line, piece.position.col);

    let vertical_distance = (line as i8 - destination.line as i8).abs();
    let horizontal_distance = (col as i8 - destination.col as i8).abs();

    (vertical_distance == 1 && horizontal_distance == 2)
        || (vertical_distance == 2 && horizontal_distance == 1)
}
