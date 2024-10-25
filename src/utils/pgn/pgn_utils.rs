use std::str::Chars;

use crate::errors::MoveError;
use crate::game_state::GameState;
use crate::utils::constants::QUEEN_SIDE_CASTLING;
use crate::utils::types::Move;

use super::pgn_parser_steps::{Fifth, First, Fourth, Second, Third};

/// Represents all the possible steps in the PGN parse process.
/// Each state is related with a character in the PGN string.
pub enum PgnParserState {
    First(First),
    Second(Second),
    Third(Third),
    Fourth(Fourth),
    Fifth(Fifth),
    Finished,
}

/// Abstracts a state machine that parses a given PGN, step by step.
/// Each step parses a given positional character, according to its name.
pub struct PgnParser<'a, 'b> {
    pub game_state: &'a GameState,
    pub pgn_chars: Chars<'b>,
    pub next_move: Option<Move>,
    /// This should be only used for castling
    pub additional_next_move: Option<Move>,
    pub state: PgnParserState,
    pub castling_chars: Chars<'b>,
}

impl<'a, 'b> PgnParser<'a, 'b> {
    fn new(game_state: &'a GameState, pgn_str: &'b str) -> Self {
        let pgn_chars = pgn_str.chars();
        let pgn_len = pgn_str.len();
        // let king_size_castling_chars = KING_SIDE_CASTLING.chars();
        let queen_size_castling_chars = QUEEN_SIDE_CASTLING.chars();

        Self {
            game_state,
            pgn_chars,
            next_move: None,
            additional_next_move: None,
            state: PgnParserState::First(First { pgn_len }),
            // king_size_castling_chars,
            castling_chars: queen_size_castling_chars,
        }
    }
}

impl<'a, 'b> PgnParser<'a, 'b> {
    fn step(&mut self) -> Result<(), MoveError> {
        match self.state {
            // TODO Maybe if the PgnParserState is a trait, we don't need to have all this boilerplate?
            // TODO we could define the `parse` method in the trait
            // TODO Another idea: we can keep using the PgnParserState as an enum and create another
            // TODO struct member to store each struct state that implements a trait with the `parse` method
            PgnParserState::First(first) => first.parse(self),
            PgnParserState::Second(second) => second.parse(self),
            PgnParserState::Third(third) => third.parse(self),
            PgnParserState::Fourth(fourth) => fourth.parse(self),
            PgnParserState::Fifth(fifth) => fifth.parse(self),
            PgnParserState::Finished => {
                panic!("Logic error! There should be no step from the 'Finished' state.")
            }
        }
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
