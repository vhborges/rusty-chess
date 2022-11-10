use super::constants::{BOARD_SIZE, LINE_RANGE, COL_RANGE};
use crate::errors::{PositionError, ChessPositionError};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

pub struct ChessPosition {
    pub line: char,
    pub col: char,
}

impl ChessPosition {
    pub fn new(line: char, col: char) -> Self {
        Self { line, col }
    }
}

impl TryFrom<ChessPosition> for Position {
    type Error = ChessPositionError;

    fn try_from(chess_pos: ChessPosition) -> Result<Self, Self::Error> {
        if !LINE_RANGE.contains(&chess_pos.line) {
            return Err(ChessPositionError::InvalidLine(chess_pos.line));
        }
        if !COL_RANGE.contains(&chess_pos.col) {
            return Err(ChessPositionError::InvalidColumn(chess_pos.col));
        }

        let chess_line = chess_pos.line.to_digit(10).unwrap();

        let line = BOARD_SIZE - chess_line as usize;
        let col = chess_pos.col as usize - 'a' as usize;

        Ok(Position::new(line, col))
    }
}

impl TryFrom<Position> for ChessPosition {
    type Error = PositionError;

    fn try_from(position: Position) -> Result<Self, Self::Error> {
        if !(0..BOARD_SIZE).contains(&position.line) {
            return Err(PositionError::InvalidLine(position.line));
        }
        if !(0..BOARD_SIZE).contains(&position.col) {
            return Err(PositionError::InvalidColumn(position.col));
        }

        let chess_line = char::from((BOARD_SIZE - position.line) as u8);
        let chess_col = (position.col as u8 + 'a' as u8) as char;

        Ok(ChessPosition::new(chess_line, chess_col))
    }
}
