use crate::piece::Piece;
use crate::utils::constants::{
    BLACK_CASTLING_LINE, KING_INITIAL_COLUMN, KING_LONG_CASTLING_COLUMN,
    KING_SHORT_CASTLING_COLUMN, WHITE_CASTLING_LINE,
};
use crate::utils::{Board, Color, Position};

pub const SYMBOLS: [char; 2] = ['\u{2654}', '\u{265A}'];

pub fn can_move(origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    let horizontal_distance = dest_col - src_col;
    let vertical_distance = dest_line - src_line;

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
            if board[i][j].is_some() {
                return false;
            }
        }
    }
    else {
        for j in 1..4 {
            if board[i][j].is_some() {
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
