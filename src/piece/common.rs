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

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub short_castling_available: bool,
    pub long_castling_available: bool,
    symbol: char,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        let castling_available = matches!(piece_type, PieceType::King | PieceType::Rook);
        Self {
            piece_type,
            color,
            short_castling_available: castling_available,
            long_castling_available: castling_available,
            symbol: Self::get_symbol(&piece_type, &color),
        }
    }
    
    pub fn can_castle(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
    ) -> Result<bool, MoveError> {
        Self::validate_move(origin, destination)?;

        match self.piece_type {
            PieceType::King => Ok(king::can_castle(self, board, origin, destination)),
            PieceType::Rook => Ok(rook::can_castle(self, board, origin, destination)),
            _ => Err(MoveError::InvalidCastle("Invalid piece for castling"))
        }
    }

    pub fn can_move(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
    ) -> Result<bool, MoveError> {
        Self::validate_move(origin, destination)?;
        self.validate_capture(&board[destination.line][destination.col], false)?;

        match self.piece_type {
            PieceType::Bishop => Ok(bishop::can_move(board, origin, destination)),
            PieceType::King => Ok(king::can_move(origin, destination)),
            PieceType::Knight => Ok(knight::can_move(origin, destination)),
            PieceType::Pawn => Ok(pawn::can_move(self, board, origin, destination)),
            PieceType::Queen => Ok(queen::can_move(board, origin, destination)),
            PieceType::Rook => Ok(rook::can_move(board, origin, destination)),
            // TODO fill panic message
            PieceType::None => panic!(),
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
        // TODO review this logic
        if capture {
            // TODO this function purpose is not clear considering the last argument
            self.validate_capture(&board[destination.line][destination.col], true)?;
        }

        match self.piece_type {
            PieceType::Bishop => Ok(bishop::attacks(board, origin, destination)),
            PieceType::King => Ok(king::attacks(origin, destination)),
            PieceType::Knight => Ok(knight::attacks(origin, destination)),
            PieceType::Pawn => Ok(pawn::attacks(self.color, origin, destination)),
            PieceType::Queen => Ok(queen::attacks(board, origin, destination)),
            PieceType::Rook => Ok(rook::attacks(board, origin, destination)),
            // TODO fill panic message
            PieceType::None => panic!(),
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
            return Err(MoveError::InvalidMove(
                "The piece should not stay where it is",
            ));
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
            // TODO fill panic message
            PieceType::None => panic!(),
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
