use crate::Board;
use crate::movement::Move;
use std::str::Chars;

pub fn perform_move(move_: Move, board: &mut Board) {
    if move_.source() != move_.destination() {
        board.move_piece(move_.source(), move_.destination());
    }

    if let Some(additional) = move_.additional {
        let source = additional.source;
        let dest = additional.destination;
        if source != dest {
            board.move_piece(source, dest);
        }
    }
}

pub fn get_next_char(line: &String, chars: &mut Chars) -> char {
    chars
        .next()
        .unwrap_or_else(|| panic!("Line {line} is incomplete"))
}
