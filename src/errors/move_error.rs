use std::error::Error;
use std::fmt::Display;

use super::PositionError;

#[derive(Debug)]
pub enum MoveError {
    InvalidMove(String),
    MissingPiece,
    InvalidSquare(String),
    InvalidPosition(PositionError),
}

impl Error for MoveError {}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMove(err) => write!(f, "Invalid move: {}", err),
            Self::MissingPiece => write!(f, "Piece type expected, e.g. B for Bishop"),
            Self::InvalidSquare(square) => write!(
                f,
                "Invalid square {}. Expected 2 characters, e.g. d2",
                square
            ),
            Self::InvalidPosition(position_err) => write!(
                f,
                "Invalid position: {}",
                position_err
            ),
        }
    }
}

impl From<PositionError> for MoveError {
    fn from(position_err: PositionError) -> Self {
        Self::InvalidPosition(position_err)
    }
}

impl From<String> for MoveError {
    fn from(err: String) -> Self {
        Self::InvalidMove(err)
    }
}
