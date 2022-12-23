use crate::utils::{Color, Position};

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

pub fn can_move(piece: Piece, destination: Position, capture: bool) -> bool {
    let (src_line, src_col) = (piece.position.line as i8, piece.position.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    let mut allow_two_rows = false;

    let vertical_distance: i8;
    match piece.color {
        Color::White => {
            if src_line == 6 {
                allow_two_rows = true;
            }
            vertical_distance = src_line - dest_line;
        }
        Color::Black => {
            if src_line == 1 {
                allow_two_rows = true;
            }
            vertical_distance = dest_line - src_line;
        }
    }
    let horizontal_distance = (src_col - dest_col).abs();

    if capture {
        return vertical_distance == 1 && horizontal_distance == 1;
    }

    if dest_col != src_col {
        return false;
    }

    return vertical_distance == 1 || (allow_two_rows && vertical_distance == 2);
}
