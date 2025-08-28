use crate::io::file_manager::initial_positions;
use crate::movement::ChessPosition;
use crate::pieces::utils::Color;
use crate::pieces::{Piece, PieceType};
use crate::utils::helper_functions::get_next_char;
use crate::{Board, GameState};

pub fn setup_game_state(positions_file: Option<&str>) -> GameState {
    let mut game_state = GameState::new();
    game_state.initialize(positions_file);
    game_state
}

pub fn setup_board(positions_file: Option<&str>) -> Board {
    let mut board = Board::new();
    for wrapped_line in initial_positions(positions_file) {
        let line = wrapped_line.expect("Error reading file line");
        let mut chars = line.chars();

        let piece_color: Color = get_next_char(&line, &mut chars)
            .try_into()
            .unwrap_or_else(|_| panic!("Could not parse color character from line {line}"));

        let piece_type: PieceType = get_next_char(&line, &mut chars)
            .try_into()
            .unwrap_or_else(|_| panic!("Could not parse piece character from line {line}"));

        let chess_col = get_next_char(&line, &mut chars);

        let chess_line = get_next_char(&line, &mut chars);

        let piece_position = ChessPosition::new(chess_line, chess_col)
            .try_into()
            .unwrap_or_else(|_| {
                panic!("Could not convert ChessPosition {chess_col}{chess_line} to Position")
            });

        board.add_piece(Piece::new(piece_type, piece_color), piece_position);
    }

    board
}
