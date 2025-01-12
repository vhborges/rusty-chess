use chess::io::ui::read_move;
use chess::GameState;

fn main() {
    let mut game_state = GameState::new();

    game_state.initialize(None);
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
