use crate::errors::PgnError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
    None,
}

impl TryFrom<char> for PieceType {
    type Error = PgnError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' => Ok(PieceType::Bishop),
            'K' => Ok(PieceType::King),
            'N' => Ok(PieceType::Knight),
            'a'..='h' | 'P' => Ok(PieceType::Pawn),
            'Q' => Ok(PieceType::Queen),
            'R' => Ok(PieceType::Rook),
            'O' => Ok(PieceType::None),
            _ => Err(PgnError::InvalidPiece(value)),
        }
    }
}
