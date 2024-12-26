use crate::piece::Piece;
use crate::utils::constants::{BOARD_SIZE, INTERNAL_ERROR_04};

use super::Position;

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

#[derive(Copy, Clone)]
pub struct Move {
    pub source: Position,
    pub destination: Position,
    
    // The following should be only used for castling
    pub opt_source: Option<Position>,
    pub opt_destination: Option<Position>,
}

impl Move {
    pub fn new(
        source: Position,
        destination: Position,
    ) -> Self {
        Self {
            source,
            destination,
            opt_source: None,
            opt_destination: None,
        }
    }
    
    pub fn new_with_options(
        source: Position,
        destination: Position,
        opt_source: Position,
        opt_destination: Position,
    ) -> Self {
        Self {
            source,
            destination,
            opt_source: Some(opt_source),
            opt_destination: Some(opt_destination),
        }
    }
    
    pub fn is_castle(&self) -> bool {
        if self.opt_source.is_some() != self.opt_destination.is_some() {
            panic!("{}", INTERNAL_ERROR_04)
        }
        
        self.opt_source.is_some()
    }
}
