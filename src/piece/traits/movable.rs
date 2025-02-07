use crate::types::Position;

/// Defines common functions for piece movement
pub trait Movable {
    /// True if the given `origin` and `destination` Positions correspond to a valid move for this piece, false otherwise.
    /// This function will NOT check if the move path is clear, this should be done using the `get_move_path` function.
    fn is_valid_move(origin: Position, destination: Position) -> bool;
    
    /// True if the given `origin` and `destination` Positions correspond to a valid attack for this piece, false otherwise.
    /// This function will NOT check if the attack path is clear, this should be done using the `get_move_path` function.
    fn is_valid_attack(origin: Position, destination: Position) -> bool;
    
    /// Return a `Vec` containing the path that this piece will traverse during the move.
    fn get_move_path(source: Position, destination: Position) -> Vec<Position>;
}
