use crate::piece::PieceType;
use std::str::Chars;

/// Store common data for all steps.
pub struct ParserState<'a> {
    pub piece_type: PieceType,
    pub capture: bool,
    pub castling: bool,
    pub dest_col: Option<char>,
    pub disambiguation: Option<char>,
    pub pgn_chars: Chars<'a>,
    pub castling_chars: Chars<'static>,
}
