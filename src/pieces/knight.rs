use crate::utils::Position;

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2658}', '\u{265E}'];

pub fn can_move(piece: Piece, destination: Position) -> bool {
    let (src_line, src_col) = (piece.position.line as i8, piece.position.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    let vertical_distance = (src_line - dest_line).abs();
    let horizontal_distance = (src_col - dest_col).abs();

    (vertical_distance == 1 && horizontal_distance == 2) ||
        (vertical_distance == 2 && horizontal_distance == 1)
}
