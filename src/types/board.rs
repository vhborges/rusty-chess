use crate::piece::Piece;
use crate::types::Position;
use crate::utils::constants::BLANK_SQUARE;
use std::ops::RangeInclusive;

pub const BOARD_SIZE: usize = 8;

pub const LINES: [char; BOARD_SIZE] = ['8', '7', '6', '5', '4', '3', '2', '1'];
pub const COLUMNS: [char; BOARD_SIZE] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub const LINE_RANGE: RangeInclusive<char> = '1'..='8';
pub const COL_RANGE: RangeInclusive<char> = 'a'..='h';

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

    pub fn is_path_clear(&self, path: Vec<Position>) -> bool {
        for pos in path {
            if self.board[pos.line][pos.col].is_some() {
                return false;
            }
        }
        true
    }

    pub fn print_board(&self) {
        for (line, line_chess) in (0..BOARD_SIZE).zip(LINES.iter()) {
            print!("{} ", line_chess);
            for column in 0..BOARD_SIZE {
                let maybe_piece = self.board[line][column];
                match maybe_piece {
                    Some(piece) => print!("{} ", piece),
                    None => print!("{} ", BLANK_SQUARE),
                }
            }
            println!();
        }

        print!("  ");

        for col_chess in COLUMNS {
            print!("{} ", col_chess);
        }
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
