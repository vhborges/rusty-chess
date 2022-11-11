use std::error::Error;
use std::fmt::Display;

use super::{ChessPositionError, PositionError};

#[derive(Debug, PartialEq)]
pub enum MoveError {
    // TODO convert to &str
    InvalidMove(String),
    InvalidCharacter(char),
    InvalidPosition(PositionError),
    InvalidChessPosition(ChessPositionError),
}

impl Error for MoveError {}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMove(err) => write!(f, "Invalid move: {}", err),
            Self::InvalidCharacter(square) => write!(f, "Invalid character: {}", square),
            Self::InvalidPosition(position_err) => write!(f, "Invalid position: {}", position_err),
            Self::InvalidChessPosition(chess_position_err) => {
                write!(f, "Invalid Chess position: {}", chess_position_err)
            }
        }
    }
}

impl From<PositionError> for MoveError {
    fn from(position_err: PositionError) -> Self {
        Self::InvalidPosition(position_err)
    }
}

impl From<ChessPositionError> for MoveError {
    fn from(chess_position_err: ChessPositionError) -> Self {
        Self::InvalidChessPosition(chess_position_err)
    }
}

impl From<String> for MoveError {
    fn from(err: String) -> Self {
        Self::InvalidMove(err)
    }
}
