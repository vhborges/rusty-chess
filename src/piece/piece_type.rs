use crate::errors::PgnError;
use crate::piece::pieces::{bishop, king, knight, pawn, queen, rook};
use crate::types::{Board, Color, Position};
use crate::utils::constants::INTERNAL_ERROR_04;

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

impl PieceType {
    pub fn can_move(
        &self,
        origin: Position,
        destination: Position,
    ) -> bool {
        match self {
            PieceType::Bishop => bishop::can_move(origin, destination),
            PieceType::King => king::can_move(origin, destination),
            PieceType::Knight => knight::can_move(origin, destination),
            PieceType::Pawn => pawn::can_move(origin, destination),
            PieceType::Queen => queen::can_move(origin, destination),
            PieceType::Rook => rook::can_move(origin, destination),
            PieceType::None => panic!("{}", INTERNAL_ERROR_04),
        }
    }
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
