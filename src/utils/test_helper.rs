use crate::GameState;

pub fn setup(positions_file: Option<&str>) -> GameState {
    let mut game_state = GameState::new();
    game_state.initialize(positions_file);
    game_state
}
