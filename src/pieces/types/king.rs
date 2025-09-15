use super::super::Color;
use crate::Board;
use crate::board::constants::{BLACK_CASTLING_LINE, WHITE_CASTLING_LINE};
use crate::movement::{Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2654}', '\u{265A}'];

pub const KING_SHORT_CASTLING_COLUMN: usize = 6;
pub const KING_LONG_CASTLING_COLUMN: usize = 2;
pub const KING_INITIAL_COLUMN: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct King {
    pub short_castling_available: bool,
    pub long_castling_available: bool,
}

impl Default for King {
    fn default() -> Self {
        Self::new()
    }
}

impl King {
    pub fn new() -> Self {
        Self {
            short_castling_available: true,
            long_castling_available: true,
        }
    }

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

    pub fn can_castle(
        &self,
        piece_color: Color,
        board: &Board,
        origin: Position,
        destination: Position,
    ) -> bool {
        if destination.col != 2 && destination.col != 6 {
            return false;
        }
        if destination.col == 6 && !self.short_castling_available {
            return false;
        }
        if destination.col == 2 && !self.long_castling_available {
            return false;
        }

        match piece_color {
            Color::White => {
                if destination.line != 7 {
                    return false;
                }
                if origin.line != 7 && origin.col != 4 {
                    return false;
                }
                let i = 7;
                Self::check_clear_path(board, destination, i)
            }
            Color::Black => {
                if destination.line != 0 {
                    return false;
                }
                if origin.line != 0 && origin.col != 4 {
                    return false;
                }
                let i = 0;
                Self::check_clear_path(board, destination, i)
            }
        }
    }

    pub fn attacks(origin: Position, destination: Position) -> bool {
        Self::can_move(origin, destination)
    }

    // TODO unit test this function with a source in the edges of the board
    pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
        let mut result = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let dest_i8 = PositionI8::new(source.line as i8 + dy, source.col as i8 + dx);
                if let Ok(dest) = dest_i8.try_into()
                    && !board.is_position_occupied(dest)
                {
                    result.push(dest);
                }
            }
        }

        result
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

    pub fn deny_castling_rights(&mut self) {
        self.short_castling_available = false;
        self.long_castling_available = false;
    }
}
