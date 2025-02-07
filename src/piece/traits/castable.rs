use crate::piece::Piece;
use crate::types::{Color, Position};

/// Defines a piece that is eligible for castling
pub trait Castable {
    /// True if the given `origin` and `destination` Positions correspond to a valid castling move for this piece, false otherwise.
    /// This function will NOT check if the castling path is clear, this should be done using the `get_castling_path` function.
    fn is_valid_castle(piece: &Piece, origin: Position, destination: Position) -> bool;
    
    /// Return a `Vec` containing the path that this piece will traverse during the castling move.
    fn get_castling_path(short: bool, color: Color) -> Vec<Position>;
}
