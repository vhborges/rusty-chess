use crate::board::constants::{BOARD_SIZE, COL_RANGE, LINE_RANGE};
use crate::errors::{ChessPositionError, PositionError};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PositionI8 {
    pub line: i8,
    pub col: i8,
}

impl PositionI8 {
    pub fn new(line: i8, col: i8) -> Self {
        Self { line, col }
    }

    pub fn is_position_valid(&self) -> bool {
        (0..BOARD_SIZE as i8).contains(&self.line) && (0..BOARD_SIZE as i8).contains(&self.col)
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

        let line = BOARD_SIZE - (chess_line as usize);
        let col = (chess_pos.col as usize) - ('a' as usize);

        Ok(Position::new(line, col))
    }
}

impl TryFrom<Position> for ChessPosition {
    type Error = PositionError;

    fn try_from(position: Position) -> Result<Self, Self::Error> {
        if !(0..BOARD_SIZE).contains(&position.line) {
            return Err(PositionError::InvalidLine(position.line as i8));
        }
        if !(0..BOARD_SIZE).contains(&position.col) {
            return Err(PositionError::InvalidColumn(position.col as i8));
        }

        let chess_line = ((BOARD_SIZE - position.line) as u8 + b'0') as char;
        let chess_col = ((position.col as u8) + b'a') as char;

        Ok(ChessPosition::new(chess_line, chess_col))
    }
}

impl From<Position> for PositionI8 {
    fn from(position: Position) -> Self {
        PositionI8::new(position.line as i8, position.col as i8)
    }
}

impl Iterator for Position {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line == (BOARD_SIZE - 1) && self.col == (BOARD_SIZE - 1) {
            return None;
        }

        if self.col == BOARD_SIZE - 1 {
            self.col = 0;
            self.line += 1;
        }
        else {
            self.col += 1;
        }

        Some(*self)
    }
}

impl TryFrom<PositionI8> for Position {
    type Error = PositionError;

    fn try_from(value: PositionI8) -> Result<Self, Self::Error> {
        if !(0..BOARD_SIZE as i8).contains(&value.line) {
            return Err(PositionError::InvalidLine(value.line));
        }
        if !(0..BOARD_SIZE as i8).contains(&value.col) {
            return Err(PositionError::InvalidColumn(value.col));
        }

        Ok(Position::new(value.line as usize, value.col as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chess_position_try_from() -> Result<(), PositionError> {
        let position = Position::new(6, 1);
        let chess_pos = ChessPosition::try_from(position)?;

        assert_eq!(chess_pos.line, '2');
        assert_eq!(chess_pos.col, 'b');

        Ok(())
    }

    #[test]
    fn test_position_try_from() -> Result<(), ChessPositionError> {
        let chess_position = ChessPosition::new('3', 'c');
        let position = Position::try_from(chess_position)?;

        assert_eq!(position.line, 5);
        assert_eq!(position.col, 2);

        Ok(())
    }
}
