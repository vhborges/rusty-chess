use crate::utils::types::Board;
use crate::utils::{Color, Position};

use super::Piece;

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

pub fn can_move(board: &Board, piece: &Piece, origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
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

    if dest_col != src_col {
        return false;
    }

    vertical_distance == 1
        || (allow_two_rows
            && vertical_distance == 2
            && check_clear_path(board, origin, destination))
}

fn check_clear_path(board: &Board, origin: Position, destination: Position) -> bool {
    let middle_line = (origin.line + destination.line) / 2;

    board[middle_line][origin.col].is_none()
}

pub fn attacks(piece_color: Color, origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    let vertical_distance = dest_line - src_line;
    let abs_horizontal_distance = (dest_col - src_col).abs();

    match piece_color {
        Color::White => vertical_distance == -1 && abs_horizontal_distance == 1,
        Color::Black => vertical_distance == 1 && abs_horizontal_distance == 1,
    }
}
