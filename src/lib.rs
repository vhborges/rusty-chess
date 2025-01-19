pub mod errors;
mod game_state;
mod io;
mod pgn;
mod piece;
pub mod utils;

pub use game_state::GameState;

use crate::io::ui::read_move;

pub fn run(game_state: &mut GameState) {
    game_state.print_game();

    loop {
        match read_move() {
            Ok(next_move) => {
                if let Err(move_err) = game_state.handle_move(next_move.as_str()) {
                    game_state.print_game();
                    println!("{}", next_move);
                    println!("{}", move_err);
                    continue;
                }
            }
            Err(move_err) => {
                game_state.print_game();
                println!("{}", move_err);
                continue;
            }
        }

        game_state.print_game();
    }
}