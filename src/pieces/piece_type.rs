use crate::errors::PgnError;
use crate::pieces::types::King as KingStruct;
use crate::pieces::types::Pawn as PawnStruct;
use crate::pieces::types::Rook as RookStruct;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Bishop,
    King(KingStruct),
    Knight,
    Pawn(PawnStruct),
    Queen,
    Rook(RookStruct),
    None,
}

impl TryFrom<char> for PieceType {
    type Error = PgnError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' => Ok(PieceType::Bishop),
            'K' => Ok(PieceType::King(Default::default())),
            'N' => Ok(PieceType::Knight),
            'a'..='h' | 'P' => Ok(PieceType::Pawn(Default::default())),
            'Q' => Ok(PieceType::Queen),
            'R' => Ok(PieceType::Rook(Default::default())),
            'O' => Ok(PieceType::None),
            _ => Err(PgnError::InvalidPiece(value)),
        }
    }
}
