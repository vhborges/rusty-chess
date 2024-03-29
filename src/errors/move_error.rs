use std::error::Error;
use std::fmt::Display;

use super::{ChessPositionError, PgnError, PositionError};

#[derive(Debug, PartialEq)]
pub enum MoveError {
    NoPieceAvailable,
    MoreThanOnePieceAvailable,
    SquareOccupied,
    KingWouldBeInCheck,
    InvalidCapture(&'static str),
    InvalidPgn(PgnError),
}

impl Error for MoveError {}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPieceAvailable => write!(f, "No piece available for this move"),
            Self::MoreThanOnePieceAvailable => {
                write!(f, "More than one piece available for this move")
            }
            Self::SquareOccupied => write!(f, "Invalid move: square already occupied"),
            Self::KingWouldBeInCheck => write!(f, "Invalid move: this would put your King in check"),
            Self::InvalidCapture(err) => write!(f, "Invalid capture: {}", err),
            Self::InvalidPgn(err) => write!(f, "Invalid PGN: {}", err),
        }
    }
}

impl From<PgnError> for MoveError {
    fn from(err: PgnError) -> Self {
        Self::InvalidPgn(err)
    }
}

impl From<ChessPositionError> for MoveError {
    fn from(err: ChessPositionError) -> Self {
        return Into::<PgnError>::into(err).into();
    }
}

impl From<PositionError> for MoveError {
    fn from(err: PositionError) -> Self {
        return Into::<PgnError>::into(err).into();
    }
}
