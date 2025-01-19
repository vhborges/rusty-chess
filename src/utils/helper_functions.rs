use crate::types::{Board, Move};
use std::str::Chars;

pub fn perform_move(move_: &Move, board: &mut Board) {
    if move_.source() != move_.destination() {
        board[move_.destination().line][move_.destination().col] =
            board[move_.source().line][move_.source().col];
        board[move_.source().line][move_.source().col] = None;
    }

    if let Some(additional) = move_.additional {
        let source = additional.source;
        let dest = additional.destination;
        if source != dest {
            board[dest.line][dest.col] = board[source.line][source.col];
            board[source.line][source.col] = None;
        }
    }
}

pub fn get_next_char(line: &String, chars: &mut Chars) -> char {
    chars
        .next()
        .unwrap_or_else(|| panic!("Line {} is incomplete", line))
}
