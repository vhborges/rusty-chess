use crate::piece::Piece;
use crate::utils::constants::BOARD_SIZE;

// TODO refactor this to be a Struct that defines methods for accessing and performing operations in the board
pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];
