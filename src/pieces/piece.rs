use super::{bishop, king, knight, pawn, queen, rook};
use crate::utils::{constants::BOARD_SIZE, Board, Color, Position};

pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

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
            PieceType::King => king::possible_movements(board),
            PieceType::Queen => queen::possible_movements(board),
            PieceType::Bishop => bishop::possible_movements(board),
            PieceType::Knight => knight::possible_movements(board),
            PieceType::Rook => rook::possible_movements(board),
            PieceType::Pawn => pawn::possible_movements(board),
        }
    }

    fn get_symbol(piece_type: &PieceType, color: &Color) -> char {
        let symbols = match piece_type {
            PieceType::King => king::SYMBOLS,
            PieceType::Queen => queen::SYMBOLS,
            PieceType::Bishop => bishop::SYMBOLS,
            PieceType::Knight => knight::SYMBOLS,
            PieceType::Rook => rook::SYMBOLS,
            PieceType::Pawn => pawn::SYMBOLS,
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
