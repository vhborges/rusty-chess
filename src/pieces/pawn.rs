use crate::utils::types::Board;
use crate::utils::Position;

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

pub fn can_move(piece: Piece, destination: Position, board: Board) -> bool {
    let (line, col) = (piece.position.line, piece.position.col);

    let (dest_line, dest_col) = (destination.line, destination.col);

    dest_col == col && (dest_line as i8 - line as i8).abs() == 1
}
