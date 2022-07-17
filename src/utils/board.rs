use crate::game_state::GameState;
use crate::pieces::{Piece, PieceType};
use crate::utils::constants::BOARD_SIZE;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::position::ChessPosition;
use super::{Color, Position};

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

const INITIAL_POSITIONS: &str = "src/utils/initial_positions.txt";

pub fn initialize(game_state: &mut GameState) {
    let file = File::open(INITIAL_POSITIONS)
        .expect(format!("Could not open file {}", INITIAL_POSITIONS).as_str());
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let _line = line.expect("Error reading file line");

        let piece_color: Color = get_nth_char(&_line, 0)
            .try_into()
            .expect(format!("Could not parse color character from line {}", _line).as_str());

        let piece_type: PieceType = get_nth_char(&_line, 1)
            .try_into()
            .expect(format!("Could not parse piece character from line {}", _line).as_str());

        let col = get_nth_char(&_line, 2);

        let line = get_nth_char(&_line, 3);

        let position = Position::try_from(ChessPosition::new(line, col)).expect(
            format!(
                "Could not convert ChessPosition {}{} to Position",
                col, line
            )
            .as_str(),
        );

        let piece = Piece::new(piece_type, piece_color, position);

        game_state.add_piece(piece)
    }
}

fn get_nth_char(line: &String, index: usize) -> char {
    line.chars()
        .nth(index)
        .expect(format!("Line {} does not have the {}nth character", line, index).as_str())
}
