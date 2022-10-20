use crate::utils::Position;

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub fn can_move(piece: Piece, destination: Position) -> bool {
    let (line, col) = (piece.position.line, piece.position.col);

    (line as i8 - destination.line as i8).abs() == (col as i8 - destination.col as i8).abs()
}
