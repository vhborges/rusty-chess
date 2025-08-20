use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum PositionError {
    InvalidLine(usize),
    InvalidColumn(usize),
}

impl Error for PositionError {}

impl Display for PositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionError::InvalidLine(line) => write!(f, "Line '{line}' not in range [0, 7]"),
            PositionError::InvalidColumn(col) => write!(f, "Column '{col}' not in range [0, 7]"),
        }
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ChessPositionError {
    InvalidLine(char),
    InvalidColumn(char),
    MissingDestinationColumn,
    MissingDestinationLine,
}

impl Error for ChessPositionError {}

impl Display for ChessPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLine(line) => write!(f, "Line '{line}' not in range [1, 8]"),
            Self::InvalidColumn(col) => write!(f, "Column '{col}' not in range [a, h]"),
            Self::MissingDestinationColumn => write!(f, "Missing destination column"),
            Self::MissingDestinationLine => write!(f, "Missing destination line"),
        }
    }
}
