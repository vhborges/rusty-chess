use std::error::Error;
use std::fmt::Display;

use super::{ChessPositionError, PgnError, PositionError};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum MoveError {
    NoPieceAvailable,
    MoreThanOnePieceAvailable,
    SquareOccupied,
    KingWouldBeInCheck,
    InvalidCapture(&'static str),
    InvalidMove(&'static str),
    InvalidPgn(PgnError),
    InvalidCastle(&'static str),
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
            Self::KingWouldBeInCheck => {
                write!(f, "Invalid move: this would put your King in check")
            }
            Self::InvalidCapture(err) => write!(f, "Invalid capture: {err}"),
            Self::InvalidMove(err) => write!(f, "Invalid move: {err}"),
            Self::InvalidPgn(err) => write!(f, "Invalid PGN: {err}"),
            Self::InvalidCastle(err) => write!(f, "Unable to castle: {err}"),
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
        Into::<PgnError>::into(err).into()
    }
}

impl From<PositionError> for MoveError {
    fn from(err: PositionError) -> Self {
        Into::<PgnError>::into(err).into()
    }
}
