use crate::piece::PieceType;
use std::str::Chars;

/// Store common data for all steps.
pub struct ParserState {
    pub piece_type: PieceType,
    pub capture: bool,
    pub castling: bool,
    pub dest_col: Option<char>,
    pub disambiguation: Option<char>,
}

impl Default for ParserState {
    fn default() -> Self {
        Self {
            piece_type: PieceType::None,
            capture: false,
            castling: false,
            dest_col: None,
            disambiguation: None,
        }
    }
}

/// Store common iterators for all steps
pub struct CommonIters<'a> {
    pub pgn_chars: Chars<'a>,
    pub castling_chars: Chars<'static>,
}

impl<'a> CommonIters<'a> {
    pub fn new(pgn_chars: Chars<'a>, castling_chars: Chars<'static>) -> Self {
        Self {
            pgn_chars,
            castling_chars,
        }
    }
}
