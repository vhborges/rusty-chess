mod game_state;
mod pieces;
mod utils;

use game_state::GameState;
use utils::{board, ui};

fn main() {
    let mut game_state = GameState::new();

    board::initialize(&mut game_state);

    ui::print_board(game_state);
}
