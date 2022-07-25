use std::fmt::Display;

use super::{bishop, king, knight, pawn, queen, rook};
use crate::board::Board;
use crate::utils::{constants::BOARD_SIZE, Color, Position};

#[derive(Copy, Clone)]
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
            'P' => Ok(PieceType::Pawn),
            'Q' => Ok(PieceType::Queen),
            'R' => Ok(PieceType::Rook),
            _ => Err(format!("Invalid piece character: {}", value)),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    symbol: char,
    piece_type: PieceType,
    color: Color,
    position: Position,
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

    pub fn symbol(&self) -> &char {
        &self.symbol
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn possible_movements(&self, board: Board) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
        match self.piece_type {
            PieceType::Bishop => bishop::possible_movements(board),
            PieceType::King => king::possible_movements(board),
            PieceType::Knight => knight::possible_movements(board),
            PieceType::Pawn => pawn::possible_movements(board),
            PieceType::Queen => queen::possible_movements(board),
            PieceType::Rook => rook::possible_movements(board),
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
