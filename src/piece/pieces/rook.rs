use crate::piece::Piece;
use crate::types::board::constants::{BLACK_CASTLING_LINE, WHITE_CASTLING_LINE};
use crate::types::{Board, Color, Position, PositionI8};
use std::cmp::max;

pub const SYMBOLS: [char; 2] = ['\u{2656}', '\u{265C}'];

pub const ROOK_SHORT_CASTLING_COLUMN: usize = 5;
pub const ROOK_LONG_CASTLING_COLUMN: usize = 3;
pub const ROOK_SHORT_CASTLING_INITIAL_COLUMN: usize = 7;
pub const ROOK_LONG_CASTLING_INITIAL_COLUMN: usize = 0;

pub fn can_move(board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let nr_of_squares = max((dest.col - src.col).abs(), (dest.line - src.line).abs());

    can_move_internal(board, src, dest, nr_of_squares)
}

pub fn attacks(board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let nr_of_squares = max((dest.col - src.col).abs(), (dest.line - src.line).abs()) - 1;

    can_move_internal(board, src, dest, nr_of_squares)
}

fn can_move_internal(
    board: &Board,
    source: PositionI8,
    destination: PositionI8,
    nr_of_squares: i8,
) -> bool {
    // Logical XNOR
    if (source.line == destination.line) == (source.col == destination.col) {
        return false;
    }

    board.is_path_clear(source, destination, nr_of_squares)
}

pub fn can_castle(piece: &Piece, board: &Board, origin: Position, destination: Position) -> bool {
    if !is_valid_castling(piece, origin) {
        return false;
    }

    can_move(board, origin, destination)
}

fn is_valid_castling(piece: &Piece, origin: Position) -> bool {
    if origin.col == ROOK_SHORT_CASTLING_INITIAL_COLUMN {
        if !piece.short_castling_available {
            return false;
        }
    }
    else if origin.col == ROOK_LONG_CASTLING_INITIAL_COLUMN {
        if !piece.long_castling_available {
            return false;
        }
    }
    else {
        return false;
    }
    true
}

pub fn get_castle_move(turn: Color, is_short_castle: bool) -> (Position, Position) {
    let src_line = match turn {
        Color::White => WHITE_CASTLING_LINE,
        Color::Black => BLACK_CASTLING_LINE,
    };
    let dest_line = src_line;

    let (src_col, dest_col) = match is_short_castle {
        true => (
            ROOK_SHORT_CASTLING_INITIAL_COLUMN,
            ROOK_SHORT_CASTLING_COLUMN,
        ),
        false => (ROOK_LONG_CASTLING_INITIAL_COLUMN, ROOK_LONG_CASTLING_COLUMN),
    };

    let origin = Position::new(src_line, src_col);
    let destination = Position::new(dest_line, dest_col);

    (origin, destination)
}
