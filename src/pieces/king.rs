use super::Piece;
use crate::game_state::Board;
use crate::utils::{constants::board, Color, Position};

const SYMBOLS: [char; 2] = ['\u{2654}', '\u{265A}'];

pub struct King {
    symbol: char,
    color: Color,
    position: Position,
}

impl King {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            symbol: Self::get_symbol(SYMBOLS, &color),
            color,
            position,
        }
    }
}

impl Piece for King {
    fn color(&self) -> &Color {
        &self.color
    }

    fn symbol(&self) -> &char {
        &self.symbol
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn possible_movements(&self, board: Board) -> [[bool; board::SIZE]; board::SIZE] {
        unimplemented!();
    }
}
