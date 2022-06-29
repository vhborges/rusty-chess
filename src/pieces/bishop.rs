use super::Piece;
use crate::game_state::Board;
use crate::utils::constants::board;
use crate::utils::Color;

const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub struct Bishop {
    symbol: char,
    color: Color,
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Self {
            symbol: Self::get_symbol(SYMBOLS, &color),
            color,
        }
    }
}

impl Piece for Bishop {
    fn color(&self) -> &Color {
        &self.color
    }

    fn symbol(&self) -> &char {
        &self.symbol
    }

    fn possible_movements(&self, board: Board) -> [[bool; board::SIZE]; board::SIZE] {
        unimplemented!();
    }
}
