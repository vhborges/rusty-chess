use std::error::Error;
use std::fmt::Display;

use super::{ChessPositionError, PositionError};

#[derive(Debug, PartialEq)]
pub enum PgnError {
    EmptyInput,
    MissingCharacter(&'static str),
    InvalidPiece(char),
    InvalidCharacter(char),
    InvalidPosition(PositionError),
    InvalidChessPosition(ChessPositionError),
}

impl Error for PgnError {}

impl Display for PgnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty input"),
            Self::MissingCharacter(position) => write!(f, "Missing {} character", position),
            Self::InvalidPiece(piece) => write!(f, "Invalid piece character: {}", piece),
            Self::InvalidCharacter(character) => write!(f, "Invalid character: {}", character),
            Self::InvalidPosition(err) => write!(f, "Invalid position: {}", err),
            Self::InvalidChessPosition(err) => write!(f, "Invalid chess position: {}", err),
        }
    }
}

impl From<PositionError> for PgnError {
    fn from(position_err: PositionError) -> Self {
        Self::InvalidPosition(position_err)
    }
}

impl From<ChessPositionError> for PgnError {
    fn from(chess_position_err: ChessPositionError) -> Self {
        Self::InvalidChessPosition(chess_position_err)
    }
}
