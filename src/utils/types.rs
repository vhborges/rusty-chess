use crate::pieces::Piece;
use crate::utils::constants::BOARD_SIZE;

use super::Position;

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

// Move = (origin, destination)
pub type Move = (Position, Position);
