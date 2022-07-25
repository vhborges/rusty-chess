use super::constants::BOARD_SIZE;
use crate::errors::PositionError;

#[derive(Copy, Clone)]
pub struct Position {
    line: usize,
    col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    pub fn line(&self) -> &usize {
        &self.line
    }

    pub fn col(&self) -> &usize {
        &self.col
    }
}

pub struct ChessPosition {
    line: char,
    col: char,
}

impl ChessPosition {
    pub fn new(line: char, col: char) -> Self {
        Self { line, col }
    }

    pub fn line(&self) -> &char {
        &self.line
    }

    pub fn col(&self) -> &char {
        &self.col
    }
}

impl TryFrom<ChessPosition> for Position {
    type Error = PositionError;

    fn try_from(chess_pos: ChessPosition) -> Result<Position, Self::Error> {
        if !('1'..'9').contains(chess_pos.line()) {
            return Err(PositionError::InvalidLine(*chess_pos.line()));
        }
        if !('a'..'i').contains(chess_pos.col()) {
            return Err(PositionError::InvalidColumn(*chess_pos.col()));
        }

        let chess_line = chess_pos.line.to_digit(10).unwrap();

        let line = BOARD_SIZE - chess_line as usize;
        let col = chess_pos.col as usize - 'a' as usize;

        Ok(Position::new(line, col))
    }
}
