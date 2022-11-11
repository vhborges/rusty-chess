use crate::utils::{Color, Position};

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

pub fn can_move(piece: Piece, destination: Position, capture: bool) -> bool {
    let (line, col) = (piece.position.line, piece.position.col);
    let (dest_line, dest_col) = (destination.line, destination.col);

    let mut allow_two_rows = false;

    let vertical_distance: i8;
    match piece.color {
        Color::White => {
            if line == 6 {
                allow_two_rows = true;
            }
            vertical_distance = line as i8 - dest_line as i8;
        }
        Color::Black => {
            if line == 1 {
                allow_two_rows = true;
            }
            vertical_distance = dest_line as i8 - line as i8;
        }
    }
    let horizontal_distance = (col as i8 - dest_col as i8).abs();

    if capture {
        return vertical_distance == 1 && horizontal_distance == 1;
    }

    if dest_col != col {
        return false;
    }

    return vertical_distance == 1 || (allow_two_rows && vertical_distance == 2);
}
