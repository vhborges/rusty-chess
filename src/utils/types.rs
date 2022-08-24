use crate::pieces::{Piece, PieceType};
use crate::utils::constants::BOARD_SIZE;

use super::Position;

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

pub struct Move {
    pub piece_type: PieceType,
    pub position: Position,
}

impl Move {
    pub fn new(piece_type: PieceType, position: Position) -> Self {
        Self {
            piece_type,
            position,
        }
    }
}