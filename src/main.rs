mod errors;
mod game_state;
mod io;
mod pieces;
mod utils;

use game_state::GameState;
use io::ui;

fn main() {
    let mut game_state = GameState::new();

    game_state.initialize();
    ui::print_board(&game_state);

    loop {
        let result = ui::read_move();
        match result {
            Ok((source, dest)) => {
                game_state.move_piece(source, dest);
            }
            Err(move_err) => {
                println!("{}", move_err);
                continue;
            }
        }

        ui::print_board(&game_state);
    }
}
