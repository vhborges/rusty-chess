mod errors;
mod game_state;
mod io;
mod pieces;
mod utils;

use game_state::GameState;
use io::ui::{print_game, read_move};

fn main() {
    let mut game_state = GameState::new();

    game_state.initialize();
    print_game(&game_state);

    loop {
        match read_move() {
            Ok(next_move) => {
                if let Err(move_err) = game_state.move_piece(next_move) {
                    println!("{}", move_err);
                    continue;
                }
            }
            Err(move_err) => {
                println!("{}", move_err);
                continue;
            }
        }

        print_game(&game_state);
    }
}
