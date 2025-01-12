use std::str::Chars;

use crate::errors::MoveError;
use crate::game_state::GameState;
use crate::utils::constants::QUEEN_SIDE_CASTLING;
use crate::utils::types::Move;

use super::pgn_parser_steps::First;

/// Defines a trait for the possible steps in the PGN parse process.
/// Each state is related with a character in the PGN string (First, Second...).
pub trait PgnParserState {
    fn parse(self, pgn_parser: &mut PgnParser) -> Result<(), MoveError>;
}

/// Abstracts a state machine that parses a given PGN, step by step.
/// Each step parses a given positional character, according to its name.
pub struct PgnParser<'a, 'b> {
    pub game_state: &'a GameState,
    pub pgn_chars: Chars<'b>,
    pub next_move: Option<Move>,
    pub state: Box<dyn PgnParserState>,
    pub castling_chars: Chars<'b>,
}

impl<'a, 'b> PgnParser<'a, 'b> {
    fn new(game_state: &'a GameState, pgn_str: &'b str) -> Self {
        let pgn_chars = pgn_str.chars();
        let pgn_len = pgn_str.len();
        let castling_chars = QUEEN_SIDE_CASTLING.chars();

        Self {
            game_state,
            pgn_chars,
            next_move: None,
            state: Box::new(First { pgn_len }),
            castling_chars,
        }
    }
}

impl<'a, 'b> PgnParser<'a, 'b> {
    fn step(&mut self) -> Result<(), MoveError> {
        self.state.parse(self)
    }
}

pub fn parse_move(game_state: &GameState, str_move: &str) -> Result<Move, MoveError> {
    let mut pgn_parser = PgnParser::new(game_state, str_move);

    pgn_parser.step()?;

    while pgn_parser.next_move.is_none() {
        pgn_parser.step()?;
    }

    Ok(pgn_parser.next_move.unwrap())
}
