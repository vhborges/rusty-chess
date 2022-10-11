use crate::pieces::{Piece, PieceType};
use crate::utils::constants::BOARD_SIZE;

use super::Position;

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

pub struct Move {
    pub source: Position,
    pub destination: Position,
}

impl Move {
    pub fn new(origin: Position, destination: Position) -> Self {
        Self {
            source: origin,
            destination,
        }
    }
}