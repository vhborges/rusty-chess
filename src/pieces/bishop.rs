use crate::utils::Position;

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub fn can_move(piece: Piece, destination: Position) -> bool {
    let (src_line, src_col) = (piece.position.line as i8, piece.position.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    (src_line - dest_line).abs() == (src_col - dest_col).abs()
}
