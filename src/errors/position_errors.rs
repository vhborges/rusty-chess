use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum PositionError {
    InvalidLine(usize),
    InvalidColumn(usize),
}

impl Error for PositionError {}

impl Display for PositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionError::InvalidLine(line) => write!(
                f,
                "Line '{}' not in range [0, 7]",
                line
            ),
            PositionError::InvalidColumn(col) => write!(
                f,
                "Column '{}' not in range [0, 7]",
                col
            ),
        }
    }
}

#[derive(Debug)]
pub enum ChessPositionError {
    InvalidLine(char),
    InvalidColumn(char),
}

impl Error for ChessPositionError {}

impl Display for ChessPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessPositionError::InvalidLine(line) => write!(
                f,
                "Line '{}' not in range [1, 8]",
                line
            ),
            ChessPositionError::InvalidColumn(col) => write!(
                f,
                "Column '{}' not in range [a, h]",
                col
            ),
        }
    }
}
