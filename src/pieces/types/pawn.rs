use super::super::Color;
use super::super::Piece;
use crate::Board;
use crate::movement::{Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pawn {
    pub allow_two_rows: bool,
}

impl Default for Pawn {
    fn default() -> Self {
        Self::new()
    }
}

impl Pawn {
    pub fn new() -> Self {
        Pawn {
            allow_two_rows: true,
        }
    }

    pub fn can_move(
        &self,
        piece: &Piece,
        board: &Board,
        source: Position,
        destination: Position,
    ) -> bool {
        let src: PositionI8 = source.into();
        let dest: PositionI8 = destination.into();

        let vertical_distance = match piece.color {
            Color::White => src.line - dest.line,
            Color::Black => dest.line - src.line,
        };

        if dest.col != src.col {
            return false;
        }

        vertical_distance == 1
            || (self.allow_two_rows && vertical_distance == 2)
                && board.is_path_clear(src, dest, vertical_distance as usize)
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

    pub fn get_possible_moves(
        &self,
        piece_color: Color,
        board: &Board,
        source: Position,
    ) -> Vec<Position> {
        let mut result = Vec::new();

        let direction = match piece_color {
            Color::White => 1,
            Color::Black => -1,
        };

        let mut dest_i8: PositionI8 = source.into();

        // 1 row
        dest_i8.line += direction;
        if let Ok(dest) = dest_i8.try_into()
            && !board.is_position_occupied(dest)
        {
            result.push(dest)
        }
        else {
            return result;
        }

        // 2 rows
        if self.allow_two_rows {
            dest_i8.line += direction;
            if let Ok(dest) = dest_i8.try_into()
                && !board.is_position_occupied(dest)
            {
                result.push(dest)
            }
        }

        result
    }
}
