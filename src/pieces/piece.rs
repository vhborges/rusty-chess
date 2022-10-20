use std::fmt::Display;

use super::{bishop, king, knight, pawn, queen, rook};
use crate::errors::MoveError;
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
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' => Ok(PieceType::Bishop),
            'K' => Ok(PieceType::King),
            'N' => Ok(PieceType::Knight),
            'a'..='h' | 'P' => Ok(PieceType::Pawn),
            'Q' => Ok(PieceType::Queen),
            'R' => Ok(PieceType::Rook),
            _ => Err(format!("Invalid piece character: {}", value)),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    symbol: char,
    pub piece_type: PieceType,
    pub color: Color,
    pub position: Position,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color, position: Position) -> Self {
        Self {
            symbol: Self::get_symbol(&piece_type, &color),
            piece_type,
            color,
            position,
        }
    }

    pub fn can_move(
        &self,
        board: Board,
        destination: Position,
        capture: bool,
    ) -> Result<bool, MoveError> {
        if capture {
            let dest_piece = board[destination.line][destination.col];
            if dest_piece.is_none() || dest_piece.unwrap().color == self.color {
                return Err(MoveError::InvalidMove("Invalid capture".to_owned()));
            }
        }

        let (line, col) = (self.position.line, self.position.col);
        assert!(
            board[line][col].is_some() && board[line][col].unwrap().piece_type == self.piece_type,
            "Internal error 01: Incorrect piece type or position"
        );

        match self.piece_type {
            PieceType::Bishop => Ok(bishop::can_move(*self, destination)),
            PieceType::King => Ok(king::can_move(*self, destination)),
            PieceType::Knight => Ok(knight::can_move(*self, destination)),
            PieceType::Pawn => Ok(pawn::can_move(*self, destination, capture)),
            PieceType::Queen => Ok(queen::can_move(*self, destination)),
            PieceType::Rook => Ok(rook::can_move(*self, destination)),
        }
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
