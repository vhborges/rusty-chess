use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum PositionError {
    InvalidLine(char),
    InvalidColumn(char),
}

impl Error for PositionError {}

impl Display for PositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionError::InvalidLine(line) => write!(
                f,
                "Line '{}' not in range [1, 8]",
                line
            ),
            PositionError::InvalidColumn(col) => write!(
                f,
                "Column '{}' not in range [a, h]",
                col
            ),
        }
    }
}
