use super::super::Color;
use crate::Board;
use crate::board::constants::{BLACK_CASTLING_LINE, WHITE_CASTLING_LINE};
use crate::movement::{Position, PositionI8};
use std::cmp::max;

pub const SYMBOLS: [char; 2] = ['\u{2656}', '\u{265C}'];

pub const ROOK_SHORT_CASTLING_COLUMN: usize = 5;
pub const ROOK_LONG_CASTLING_COLUMN: usize = 3;
pub const ROOK_SHORT_CASTLING_INITIAL_COLUMN: usize = 7;
pub const ROOK_LONG_CASTLING_INITIAL_COLUMN: usize = 0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rook {
    pub short_castling_available: bool,
    pub long_castling_available: bool,
}

impl Default for Rook {
    fn default() -> Self {
        Self::new()
    }
}

impl Rook {
    pub fn new() -> Self {
        Self {
            short_castling_available: true,
            long_castling_available: true,
        }
    }

    pub fn can_move(board: &Board, source: Position, destination: Position) -> bool {
        let src: PositionI8 = source.into();
        let dest: PositionI8 = destination.into();

        let nr_of_squares = max((dest.col - src.col).abs(), (dest.line - src.line).abs());

        Self::can_move_internal(board, src, dest, nr_of_squares)
    }

    pub fn attacks(board: &Board, source: Position, destination: Position) -> bool {
        let src: PositionI8 = source.into();
        let dest: PositionI8 = destination.into();

        let nr_of_squares = max((dest.col - src.col).abs(), (dest.line - src.line).abs()) - 1;

        Self::can_move_internal(board, src, dest, nr_of_squares)
    }

    pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
        let mut result = Vec::new();

        for direction in (-1..=1).step_by(2) {
            // Vertical
            let mut dest =
                match PositionI8::new(source.line as i8 + direction, source.col as i8).try_into() {
                    Ok(pos) => pos,
                    Err(_) => continue,
                };

            while !board.is_position_occupied(dest) {
                result.push(dest);
                dest = match PositionI8::new(dest.line as i8 + direction, dest.col as i8).try_into()
                {
                    Ok(pos) => pos,
                    Err(_) => break,
                };
            }

            // Horizontal
            dest = match PositionI8::new(source.line as i8, source.col as i8 + direction).try_into()
            {
                Ok(pos) => pos,
                Err(_) => continue,
            };

            while !board.is_position_occupied(dest) {
                result.push(dest);
                dest = match PositionI8::new(dest.line as i8, dest.col as i8 + direction).try_into()
                {
                    Ok(pos) => pos,
                    Err(_) => break,
                };
            }
        }

        result
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

    pub fn can_castle(&self, board: &Board, origin: Position, destination: Position) -> bool {
        if !self.is_valid_castling(origin) {
            return false;
        }

        Self::can_move(board, origin, destination)
    }

    fn is_valid_castling(&self, origin: Position) -> bool {
        if origin.col == ROOK_SHORT_CASTLING_INITIAL_COLUMN {
            if !self.short_castling_available {
                return false;
            }
        }
        else if origin.col == ROOK_LONG_CASTLING_INITIAL_COLUMN {
            if !self.long_castling_available {
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

    pub fn deny_castling_rights(&mut self, pos: Position) {
        if pos.col == ROOK_LONG_CASTLING_INITIAL_COLUMN {
            self.long_castling_available = false;
        }
        else if pos.col == ROOK_SHORT_CASTLING_INITIAL_COLUMN {
            self.short_castling_available = false;
        }
    }
}
