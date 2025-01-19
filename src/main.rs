use chess::{GameState, run};

fn main() {
    let mut game_state = GameState::new();
    game_state.initialize(None);

    run(&mut game_state);
}
