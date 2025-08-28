mod board;
pub mod errors;
mod game_state;
mod io;
pub mod movement;
mod pgn;
mod pieces;
mod utils;

use crate::io::ui;
pub use board::Board;
pub use game_state::GameState;

pub fn run(game_state: &mut GameState) {
    ui::print_game(game_state);

    loop {
        match ui::read_move() {
            Ok(next_move) => {
                if let Err(move_err) = game_state.handle_move(next_move.as_str()) {
                    ui::print_game(game_state);
                    println!("{next_move}");
                    println!("{move_err}");
                    continue;
                }
            }
            Err(move_err) => {
                ui::print_game(game_state);
                println!("{move_err}");
                continue;
            }
        }

        ui::print_game(game_state);
    }
}
