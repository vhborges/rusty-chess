pub mod constants {
    use std::ops::RangeInclusive;

    // TODO increase this number just to see what happens
    pub const BOARD_SIZE: usize = 8;

    pub const WHITE_CASTLING_LINE: usize = BOARD_SIZE - 1;
    pub const BLACK_CASTLING_LINE: usize = 0;

    pub const BLANK_SQUARE: char = '_';

    pub const LINES: [char; BOARD_SIZE] = ['8', '7', '6', '5', '4', '3', '2', '1'];
    pub const COLUMNS: [char; BOARD_SIZE] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    pub const LINE_RANGE: RangeInclusive<char> = '1'..='8';
    pub const COL_RANGE: RangeInclusive<char> = 'a'..='h';
}

use crate::board::constants::{BLANK_SQUARE, BOARD_SIZE, COLUMNS, LINES};
use crate::movement::{Direction, Position, PositionI8};
use crate::pieces::Piece;

type InternalBoard = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

#[derive(Copy, Clone)]
pub struct Board {
    board: InternalBoard,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: Default::default(),
        }
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        self.board[position.line][position.col]
    }

    pub fn get_piece_as_mut(&mut self, position: Position) -> Option<&mut Piece> {
        self.board[position.line][position.col].as_mut()
    }

    pub fn update_piece(&mut self, position: Position, piece: Piece) {
        self.board[position.line][position.col] = Some(piece);
    }

    pub fn add_piece(&mut self, piece: Piece, pos: Position) {
        self.board[pos.line][pos.col] = Some(piece);
    }

    pub fn is_position_occupied(&self, position: Position) -> bool {
        self.board[position.line][position.col].is_some()
    }

    pub fn move_piece(&mut self, origin: Position, destination: Position) {
        self.board[destination.line][destination.col] = self.get_piece(origin);

        self.board[origin.line][origin.col] = None;
    }

    pub fn print_board(&self) {
        for (line, line_chess) in (0..BOARD_SIZE).zip(LINES.iter()) {
            print!("{line_chess} ");
            for column in 0..BOARD_SIZE {
                let maybe_piece = self.board[line][column];
                match maybe_piece {
                    Some(piece) => print!("{piece} "),
                    None => print!("{BLANK_SQUARE} "),
                }
            }
            println!();
        }

        print!("  ");

        for col_chess in COLUMNS {
            print!("{col_chess} ");
        }
    }

    pub fn is_path_clear(
        self,
        source: PositionI8,
        destination: PositionI8,
        nr_of_squares: i8,
    ) -> bool {
        let direction = Direction::from_position_i8(source, destination);

        let mut i = (source.line + direction.vertical) as usize;
        let mut j = (source.col + direction.horizontal) as usize;
        for _ in 0..nr_of_squares {
            if self.is_position_occupied(Position::new(i, j)) {
                return false;
            }

            i = (i as i8 + direction.vertical) as usize;
            j = (j as i8 + direction.horizontal) as usize;
        }

        true
    }

    pub fn update_piece_state(&mut self, pos: Position) {
        let piece = self.get_piece_as_mut(pos).unwrap();

        piece.deny_castling_rights(pos);
        piece.deny_two_rows();
    }
}

pub struct BoardPieceIterator<'a> {
    board: &'a Board,
    curr_pos: Position,
}

impl<'a> IntoIterator for &'a Board {
    type Item = (Piece, Position);
    type IntoIter = BoardPieceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardPieceIterator {
            board: self,
            curr_pos: Default::default(),
        }
    }
}

impl Iterator for BoardPieceIterator<'_> {
    type Item = (Piece, Position);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.curr_pos.next();
        while let Some(pos) = next {
            if let Some(piece) = self.board.get_piece(pos) {
                return Some((piece, pos));
            }
            next = self.curr_pos.next();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::setup_board;

    #[test]
    fn test_update_castling_rights_king_move() {
        let mut board = setup_board(None);

        board.update_piece_state(Position::new(0, 4));

        let piece = board.get_piece(Position::new(0, 4)).unwrap();
        assert!(!piece.is_short_castling_available());
        assert!(!piece.is_long_castling_available());
    }

    #[test]
    fn test_update_castling_rights_long_rook_move() {
        let mut board = setup_board(None);

        board.update_piece_state(Position::new(0, 0));

        let piece = board.get_piece(Position::new(0, 0)).unwrap();
        assert!(piece.is_short_castling_available());
        assert!(!piece.is_long_castling_available());
    }

    #[test]
    fn test_update_castling_rights_short_rook_move() {
        let mut board = setup_board(None);

        board.update_piece_state(Position::new(0, 7));

        let piece = board.get_piece(Position::new(0, 7)).unwrap();
        assert!(!piece.is_short_castling_available());
        assert!(piece.is_long_castling_available());
    }
}
