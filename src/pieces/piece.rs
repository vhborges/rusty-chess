use crate::game_state::Board;
use crate::utils::constants::board;
use crate::utils::Color;

pub trait Piece {
    fn symbol(&self) -> &char;

    fn color(&self) -> &Color;

    fn possible_movements(&self, board: Board) -> [[bool; board::SIZE]; board::SIZE];

    fn get_symbol(symbols: [char; 2], color: &Color) -> char
    where
        Self: Sized,
    {
        match color {
            Color::White => symbols[0],
            Color::Black => symbols[1],
        }
    }
}
