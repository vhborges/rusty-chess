use crate::game_state::GameState;
use crate::io::io::{get_next_char, initial_positions};
use crate::pieces::{Piece, PieceType};
use crate::utils::{constants::BOARD_SIZE, ChessPosition, Color};

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

pub fn initialize(game_state: &mut GameState) {
    for wrapped_line in initial_positions() {
        let line = wrapped_line.expect("Error reading file line");
        let mut chars = line.chars();

        let piece_color: Color = get_next_char(&line, &mut chars)
            .try_into()
            .expect(format!("Could not parse color character from line {}", line).as_str());

        let piece_type: PieceType = get_next_char(&line, &mut chars)
            .try_into()
            .expect(format!("Could not parse piece character from line {}", line).as_str());

        let chess_col = get_next_char(&line, &mut chars);

        let chess_line = get_next_char(&line, &mut chars);

        let piece_position = ChessPosition::new(chess_line, chess_col).try_into().expect(
            format!(
                "Could not convert ChessPosition {}{} to Position",
                chess_col, chess_line
            )
            .as_str(),
        );

        game_state.add_piece(Piece::new(piece_type, piece_color, piece_position))
    }
}
