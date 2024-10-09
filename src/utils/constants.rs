use std::ops::RangeInclusive;

// TODO add and implement support for the CHECK character (+)

pub const BOARD_SIZE: usize = 8;

pub const BLANK_SQUARE: char = '_';

pub const LINES: [char; BOARD_SIZE] = ['8', '7', '6', '5', '4', '3', '2', '1'];
pub const COLUMNS: [char; BOARD_SIZE] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub const COL_RANGE: RangeInclusive<char> = 'a'..='h';
pub const LINE_RANGE: RangeInclusive<char> = '1'..='8';

pub const CAPTURE: char = 'x';
// pub const KING_SIDE_CASTLING: &str = "O-O";
pub const QUEEN_SIDE_CASTLING: &str = "O-O-O";

pub const INTERNAL_ERROR_01: &str = "Internal error 01: Invalid piece position";
pub const INTERNAL_ERROR_02: &str = "Internal error 02: piece.attacks() should not return error when capture=false";
pub const INTERNAL_ERROR_03: &str = "Internal error 03: constant castling PGN constant should have 5 characters";
