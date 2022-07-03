use crate::pieces::Piece;
use crate::utils::constants::BOARD_SIZE;

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];
