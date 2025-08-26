use super::super::Piece;
use crate::types::{Board, Color, Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

pub fn can_move(piece: &Piece, board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let mut allow_two_rows = false;

    let vertical_distance: i8;
    match piece.color {
        Color::White => {
            if src.line == 6 {
                allow_two_rows = true;
            }
            vertical_distance = src.line - dest.line;
        }
        Color::Black => {
            if src.line == 1 {
                allow_two_rows = true;
            }
            vertical_distance = dest.line - src.line;
        }
    }

    if dest.col != src.col {
        return false;
    }

    vertical_distance == 1
        || (allow_two_rows && vertical_distance == 2 && is_path_clear(board, source, destination))
}

fn is_path_clear(board: &Board, source: Position, destination: Position) -> bool {
    let middle_line = (source.line + destination.line) / 2;

    !board.is_position_occupied(Position::new(middle_line, source.col))
}

pub fn attacks(piece_color: Color, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let vertical_distance = dest.line - src.line;
    let abs_horizontal_distance = (dest.col - src.col).abs();

    match piece_color {
        Color::White => vertical_distance == -1 && abs_horizontal_distance == 1,
        Color::Black => vertical_distance == 1 && abs_horizontal_distance == 1,
    }
}
