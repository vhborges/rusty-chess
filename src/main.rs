mod board;
mod game_state;
mod io;
mod pieces;
mod utils;

use game_state::GameState;
use io::ui;

fn main() {
    let mut game_state = GameState::new();

    board::initialize(&mut game_state);
    ui::print_board(&game_state);

    loop {
        let result = ui::read_move();
        match result {
            Ok((source, dest)) => {
                game_state.move_piece(source, dest);
            }
            Err(str) => {
                println!("{}", str);
                continue;
            }
        }

        ui::print_board(&game_state);
    }
}
