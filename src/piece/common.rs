use std::fmt::Display;

use super::pieces::{bishop, king, knight, pawn, queen, rook};
use crate::errors::{MoveError, PgnError};
use crate::utils::types::Board;
use crate::utils::{Color, Position};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
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
            _ => Err(PgnError::InvalidPiece(value)),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    // TODO should come up with a logic to update these booleans after moving the king or the rooks
    pub short_castling_available: bool,
    pub long_castling_available: bool,
    symbol: char,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            short_castling_available: match piece_type {
                PieceType::King | PieceType::Rook => true,
                _ => false,
            },
            long_castling_available: match piece_type {
                PieceType::King | PieceType::Rook => true,
                _ => false,
            },
            symbol: Self::get_symbol(&piece_type, &color),
        }
    }

    pub fn can_move(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
        castling: bool,
    ) -> Result<bool, MoveError> {
        Self::validate_move(origin, destination)?;
        self.validate_capture(&board[destination.line][destination.col], false)?;

        match self.piece_type {
            PieceType::Bishop => Ok(bishop::can_move(board, origin, destination)),
            PieceType::King => match castling {
                false => Ok(king::can_move(origin, destination)),
                true => Ok(king::can_castle(self, board, origin, destination)),
            },
            PieceType::Knight => Ok(knight::can_move(origin, destination)),
            PieceType::Pawn => Ok(pawn::can_move(board, self, origin, destination)),
            PieceType::Queen => Ok(queen::can_move(board, origin, destination)),
            PieceType::Rook => Ok(rook::can_move(board, origin, destination)),
        }
    }

    // TODO check if the destination square contains a piece of the opposite color
    pub fn attacks(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
        capture: bool,
    ) -> Result<bool, MoveError> {
        Self::validate_move(origin, destination)?;
        if capture {
            self.validate_capture(&board[destination.line][destination.col], true)?;
        }

        match self.piece_type {
            PieceType::Bishop => Ok(bishop::attacks(board, origin, destination)),
            PieceType::King => Ok(king::attacks(origin, destination)),
            PieceType::Knight => Ok(knight::attacks(origin, destination)),
            PieceType::Pawn => Ok(pawn::attacks(self.color, origin, destination)),
            PieceType::Queen => Ok(queen::attacks(board, origin, destination)),
            PieceType::Rook => Ok(rook::attacks(board, origin, destination)),
        }
    }

    fn validate_capture(&self, dest_piece: &Option<Piece>, capture: bool) -> Result<(), MoveError> {
        if capture {
            if dest_piece.is_none() {
                return Err(MoveError::InvalidCapture("Destination square is empty"));
            }
            if dest_piece.unwrap().color == self.color {
                return Err(MoveError::InvalidCapture(
                    "Cannot capture a piece of the same color",
                ));
            }
        }
        else if dest_piece.is_some() && dest_piece.unwrap().color != self.color {
            return Err(PgnError::MissingCaptureCharacter.into());
        }
        else if dest_piece.is_some() && dest_piece.unwrap().color == self.color {
            return Err(MoveError::SquareOccupied);
        }

        Ok(())
    }

    fn validate_move(origin: Position, destination: Position) -> Result<(), MoveError> {
        if origin == destination {
            return Err(MoveError::InvalidMove("The piece should not stay where it is"))
        }

        Ok(())
    }

    fn get_symbol(piece_type: &PieceType, color: &Color) -> char {
        let symbols = match piece_type {
            PieceType::Bishop => bishop::SYMBOLS,
            PieceType::King => king::SYMBOLS,
            PieceType::Knight => knight::SYMBOLS,
            PieceType::Pawn => pawn::SYMBOLS,
            PieceType::Queen => queen::SYMBOLS,
            PieceType::Rook => rook::SYMBOLS,
        };

        Self::get_symbol_for_color(color, symbols)
    }

    fn get_symbol_for_color(color: &Color, symbols: [char; 2]) -> char {
        match color {
            Color::White => symbols[0],
            Color::Black => symbols[1],
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
