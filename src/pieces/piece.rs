use super::types::{King, Pawn, Rook, bishop, king, knight, pawn, queen, rook};
use super::utils::Color;
use crate::Board;
use crate::errors::constants::INTERNAL_ERROR_04;
use crate::errors::{MoveError, PgnError};
use crate::movement::Position;
use crate::pieces::piece_type::PieceType;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    symbol: char,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            symbol: Self::get_symbol(&piece_type, &color),
        }
    }

    pub fn is_short_castling_available(&self) -> bool {
        match self.piece_type {
            PieceType::King(k) => k.short_castling_available,
            PieceType::Rook(r) => r.short_castling_available,
            _ => false,
        }
    }

    pub fn is_long_castling_available(&self) -> bool {
        match self.piece_type {
            PieceType::King(k) => k.long_castling_available,
            PieceType::Rook(r) => r.long_castling_available,
            _ => false,
        }
    }

    pub fn deny_castling_rights(&mut self, pos: Position) {
        match &mut self.piece_type {
            PieceType::King(k) => k.deny_castling_rights(),
            PieceType::Rook(r) => r.deny_castling_rights(pos),
            _ => (),
        }
    }

    pub fn deny_two_rows(&mut self) {
        if let PieceType::Pawn(p) = &mut self.piece_type {
            p.allow_two_rows = false;
        }
    }

    pub fn can_castle(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
    ) -> Result<bool, MoveError> {
        self.validate_move(origin, destination, board.get_piece(destination), false)?;

        match self.piece_type {
            PieceType::King(k) => Ok(k.can_castle(self.color, board, origin, destination)),
            PieceType::Rook(r) => Ok(r.can_castle(board, origin, destination)),
            _ => Err(MoveError::InvalidCastle("Invalid piece for castling")),
        }
    }

    pub fn can_move(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
    ) -> Result<bool, MoveError> {
        self.validate_move(origin, destination, board.get_piece(destination), false)?;

        match self.piece_type {
            PieceType::Bishop => Ok(bishop::can_move(board, origin, destination)),
            PieceType::King(_) => Ok(King::can_move(origin, destination)),
            PieceType::Knight => Ok(knight::can_move(origin, destination)),
            PieceType::Pawn(p) => Ok(p.can_move(self, board, origin, destination)),
            PieceType::Queen => Ok(queen::can_move(board, origin, destination)),
            PieceType::Rook(_) => Ok(Rook::can_move(board, origin, destination)),
            PieceType::None => panic!("{}", INTERNAL_ERROR_04),
        }
    }

    pub fn attacks(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
        capture: bool,
        validated: bool,
    ) -> Result<bool, MoveError> {
        if validated {
            self.validate_move(origin, destination, board.get_piece(destination), capture)?;
        }

        match self.piece_type {
            PieceType::Bishop => Ok(bishop::attacks(board, origin, destination)),
            PieceType::King(_) => Ok(King::attacks(origin, destination)),
            PieceType::Knight => Ok(knight::attacks(origin, destination)),
            PieceType::Pawn(_) => Ok(Pawn::attacks(self.color, origin, destination)),
            PieceType::Queen => Ok(queen::attacks(board, origin, destination)),
            PieceType::Rook(_) => Ok(Rook::attacks(board, origin, destination)),
            PieceType::None => panic!("{}", INTERNAL_ERROR_04),
        }
    }

    pub fn get_possible_moves(&self, board: &Board, pos: Position) -> Vec<Position> {
        match self.piece_type {
            PieceType::Bishop => bishop::get_possible_moves(board, pos),
            PieceType::King(_) => King::get_possible_moves(board, pos),
            PieceType::Knight => knight::get_possible_moves(board, pos),
            PieceType::Pawn(p) => p.get_possible_moves(self.color, board, pos),
            PieceType::Queen => queen::get_possible_moves(board, pos),
            PieceType::Rook(_) => Rook::get_possible_moves(board, pos),
            PieceType::None => panic!("{}", INTERNAL_ERROR_04),
        }
    }

    fn validate_move(
        &self,
        origin: Position,
        destination: Position,
        dest_piece: Option<Piece>,
        capture: bool,
    ) -> Result<(), MoveError> {
        if origin == destination {
            return Err(MoveError::InvalidMove(
                "The piece should not stay where it is",
            ));
        }
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

    fn get_symbol(piece_type: &PieceType, color: &Color) -> char {
        let symbols = match piece_type {
            PieceType::Bishop => bishop::SYMBOLS,
            PieceType::King(_) => king::SYMBOLS,
            PieceType::Knight => knight::SYMBOLS,
            PieceType::Pawn(_) => pawn::SYMBOLS,
            PieceType::Queen => queen::SYMBOLS,
            PieceType::Rook(_) => rook::SYMBOLS,
            PieceType::None => panic!("{}", INTERNAL_ERROR_04),
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
