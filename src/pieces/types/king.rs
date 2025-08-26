use crate::pieces::Piece;
use crate::types::board::constants::{BLACK_CASTLING_LINE, WHITE_CASTLING_LINE};
use crate::types::{Board, Color, Position, PositionI8};
pub const SYMBOLS: [char; 2] = ['\u{2654}', '\u{265A}'];

pub const KING_SHORT_CASTLING_COLUMN: usize = 6;
pub const KING_LONG_CASTLING_COLUMN: usize = 2;
pub const KING_INITIAL_COLUMN: usize = 4;

pub fn can_move(source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let horizontal_distance = dest.col - src.col;
    let vertical_distance = dest.line - src.line;

    if (-1..=1).contains(&horizontal_distance) && (-1..=1).contains(&vertical_distance) {
        return true;
    }

    false
}

pub fn can_castle(piece: &Piece, board: &Board, origin: Position, destination: Position) -> bool {
    if destination.col != 2 && destination.col != 6 {
        return false;
    }
    if destination.col == 6 && !piece.short_castling_available {
        return false;
    }
    if destination.col == 2 && !piece.long_castling_available {
        return false;
    }

    match piece.color {
        Color::White => {
            if destination.line != 7 {
                return false;
            }
            if origin.line != 7 && origin.col != 4 {
                return false;
            }
            let i = 7;
            check_clear_path(board, destination, i)
        }
        Color::Black => {
            if destination.line != 0 {
                return false;
            }
            if origin.line != 0 && origin.col != 4 {
                return false;
            }
            let i = 0;
            check_clear_path(board, destination, i)
        }
    }
}

pub fn attacks(origin: Position, destination: Position) -> bool {
    can_move(origin, destination)
}

fn check_clear_path(board: &Board, destination: Position, i: usize) -> bool {
    if destination.col == 6 {
        for j in 5..7 {
            if board.is_position_occupied(Position::new(i, j)) {
                return false;
            }
        }
    }
    else {
        for j in 1..4 {
            if board.is_position_occupied(Position::new(i, j)) {
                return false;
            }
        }
    }

    true
}
pub fn get_castle_move(turn: Color, is_short_castle: bool) -> (Position, Position) {
    let src_line = match turn {
        Color::White => WHITE_CASTLING_LINE,
        Color::Black => BLACK_CASTLING_LINE,
    };
    let src_col = KING_INITIAL_COLUMN;

    let dest_line = src_line;
    let dest_col = match is_short_castle {
        true => KING_SHORT_CASTLING_COLUMN,
        false => KING_LONG_CASTLING_COLUMN,
    };

    let source = Position::new(src_line, src_col);
    let destination = Position::new(dest_line, dest_col);

    (source, destination)
}
