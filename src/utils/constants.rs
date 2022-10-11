use std::ops::RangeInclusive;

pub const BOARD_SIZE: usize = 8;
pub const BLANK_SQUARE: char = '_';
pub const LINES: [char; BOARD_SIZE] = ['8', '7', '6', '5', '4', '3', '2', '1'];
pub const COLUMNS: [char; BOARD_SIZE] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const COL_RANGE: RangeInclusive<char> = 'a'..='h';
pub const LINE_RANGE: RangeInclusive<char> = '1'..='8';
pub const CAPTURE: char = 'x';
