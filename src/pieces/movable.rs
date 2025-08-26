use crate::types::{Board, Position};

pub trait Movable {
    fn can_move(board: &Board, source: Position, destination: Position) -> bool;
    fn attacks(board: &Board, source: Position, destination: Position) -> bool;
}