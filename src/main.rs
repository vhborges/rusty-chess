mod game_state;
mod pieces;
mod utils;

use game_state::GameState;
use pieces::{Piece, PieceType};
use utils::{ui, Color, Position};

fn main() {
    let mut game_state = GameState::new();

    game_state.add_piece(Piece::new(
        PieceType::Bishop,
        Color::Black,
        Position::new(5, 5),
    ));
    game_state.add_piece(Piece::new(
        PieceType::Bishop,
        Color::Black,
        Position::new(0, 0),
    ));
    game_state.add_piece(Piece::new(
        PieceType::Bishop,
        Color::White,
        Position::new(2, 2),
    ));
    game_state.add_piece(Piece::new(
        PieceType::Bishop,
        Color::White,
        Position::new(3, 3),
    ));

    ui::print_board(game_state);
}
