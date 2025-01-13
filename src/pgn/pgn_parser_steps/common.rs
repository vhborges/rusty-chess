use crate::piece::PieceType;

pub struct ParserState {
    pub piece_type: PieceType,
    pub capture: bool,
    pub castling: bool,
    pub dest_col: Option<char>,
    pub disambiguation: Option<char>,
}
