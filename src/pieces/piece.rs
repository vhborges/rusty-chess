use std::fmt::Display;

use super::{bishop, king, knight, pawn, queen, rook};
use crate::utils::types::Board;
use crate::utils::{Color, Position};

#[derive(Copy, Clone, PartialEq)]
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

    pub fn can_move(&self, board: Board, destination: Position) -> bool {
        match self.piece_type {
            PieceType::Bishop => bishop::can_move(board),
            PieceType::King => king::can_move(self.position, destination, board),
            PieceType::Knight => knight::can_move(board),
            PieceType::Pawn => pawn::can_move(self.position, destination, board),
            PieceType::Queen => queen::can_move(board),
            PieceType::Rook => rook::can_move(board),
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
