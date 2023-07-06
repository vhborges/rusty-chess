use std::str::Chars;

use crate::{
    errors::{ChessPositionError, MoveError, PgnError},
    game_state::GameState,
    pieces::PieceType,
};

use super::{
    constants::{CAPTURE, COL_RANGE, LINE_RANGE},
    types::Move,
    ChessPosition,
};

enum PgnParserState {
    First(First),
    Second(Second),
    Third(Third),
    Fourth(Fourth),
    Fifth(Fifth),
    Finished,
}

/// Abstracts a state machine that parses a given PGN, step by step.
/// Each step parses a given positional character, according to its name.
struct PgnParser<'a, 'b> {
    game_state: &'a GameState,
    pgn_chars: Chars<'b>,
    next_move: Option<Move>,
    state: PgnParserState,
}

/// First step.
#[derive(Copy, Clone)]
struct First {
    pgn_len: usize,
}

/// Second step.
#[derive(Copy, Clone)]
struct Second {
    pgn_len: usize,
    pgn_first_char: char,
    dest_col: Option<char>,
    piece_type: PieceType,
}

/// Third step.
#[derive(Copy, Clone)]
struct Third {
    capture: bool,
    disambiguation: Option<char>,
    dest_col: Option<char>,
    piece_type: PieceType,
}

/// Fourth step.
#[derive(Copy, Clone)]
struct Fourth {
    capture: bool,
    disambiguation: Option<char>,
    dest_col: Option<char>,
    piece_type: PieceType,
}

/// Fifth step.
#[derive(Copy, Clone)]
struct Fifth {
    capture: bool,
    disambiguation: Option<char>,
    dest_col: Option<char>,
    piece_type: PieceType,
}

impl<'a, 'b> PgnParser<'a, 'b> {
    /// Initializes the first step.
    fn new(game_state: &'a GameState, pgn_str: &'b str) -> Self {
        let pgn_chars = pgn_str.chars();
        let pgn_len = pgn_str.len();

        Self {
            game_state,
            pgn_chars,
            next_move: None,
            state: PgnParserState::First(First { pgn_len }),
        }
    }
}

impl<'a, 'b> PgnParser<'a, 'b> {
    fn step(&mut self) -> Result<(), MoveError> {
        match self.state {
            PgnParserState::First(first) => self.parse_first_char(first),
            PgnParserState::Second(second) => self.parse_second_char(second),
            PgnParserState::Third(third) => self.parse_third_char(third),
            PgnParserState::Fourth(fourth) => self.parse_fourth_char(fourth),
            PgnParserState::Fifth(fifth) => self.parse_fifth_char(fifth),
            PgnParserState::Finished => {
                panic!("Logic error! There should be no step from the 'Finished' state.")
            }
        }
    }

    fn parse_first_char(&mut self, first: First) -> Result<(), MoveError> {
        let dest_col;

        let current_pgn_char = self.pgn_chars.next().ok_or(PgnError::EmptyInput)?;
        let piece_type: PieceType = current_pgn_char.try_into()?;

        if piece_type == PieceType::Pawn {
            dest_col = Some(current_pgn_char);
        }
        else {
            dest_col = None;
        }

        self.state = PgnParserState::Second(Second {
            pgn_len: first.pgn_len,
            pgn_first_char: current_pgn_char,
            dest_col,
            piece_type,
        });

        Ok(())
    }

    fn parse_second_char(&mut self, second: Second) -> Result<(), MoveError> {
        let disambiguation;
        let capture;
        let piece_type = second.piece_type;
        let mut dest_col = second.dest_col;

        let current_pgn_char = self
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("second"))?;

        if current_pgn_char.is_digit(10) {
            let Some(col) = second.dest_col else {
                return Err(ChessPositionError::MissingDestinationColumn.into());
            };

            let dest_line = current_pgn_char;
            let destination = ChessPosition::new(dest_line, col).try_into()?;

            disambiguation = None;
            capture = false;

            let origin = self.game_state.find_piece_position(
                piece_type,
                destination,
                disambiguation,
                capture,
            )?;

            self.next_move = Some(Move::new(origin, destination));
        }
        else if current_pgn_char == CAPTURE {
            capture = true;

            if piece_type == PieceType::Pawn {
                disambiguation = Some(second.pgn_first_char);
            }
            else {
                disambiguation = None;
            }

            self.next_move = None;
        }
        else if second.pgn_len > 3
            && piece_type != PieceType::Pawn
            && (LINE_RANGE.contains(&current_pgn_char) || COL_RANGE.contains(&current_pgn_char))
        {
            disambiguation = Some(current_pgn_char);
            capture = false;
            self.next_move = None;
        }
        else if current_pgn_char.is_lowercase() {
            dest_col = Some(current_pgn_char);
            disambiguation = None;
            capture = false;
            self.next_move = None;
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        self.state = PgnParserState::Third(Third {
            capture,
            disambiguation,
            dest_col,
            piece_type,
        });

        Ok(())
    }

    fn parse_third_char(&mut self, third: Third) -> Result<(), MoveError> {
        let piece_type = third.piece_type;
        let mut dest_col = third.dest_col;
        let mut capture = third.capture;
        let disambiguation = third.disambiguation;

        let current_pgn_char = self
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("third"))?;

        if current_pgn_char == CAPTURE {
            capture = true;
            self.next_move = None;
        }
        else if current_pgn_char.is_digit(10) && piece_type != PieceType::Pawn {
            let Some(col) = dest_col else {
                return Err(ChessPositionError::MissingDestinationColumn.into());
            };

            let dest_line = current_pgn_char;
            let destination = ChessPosition::new(dest_line, col).try_into()?;
            let origin = self.game_state.find_piece_position(
                piece_type,
                destination,
                disambiguation,
                capture,
            )?;

            self.next_move = Some(Move::new(origin, destination));
        }
        else if current_pgn_char.is_lowercase() {
            dest_col = Some(current_pgn_char);
            self.next_move = None;
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        self.state = PgnParserState::Fourth(Fourth {
            capture,
            disambiguation,
            dest_col,
            piece_type,
        });

        Ok(())
    }

    fn parse_fourth_char(&mut self, fourth: Fourth) -> Result<(), MoveError> {
        let mut dest_col = fourth.dest_col;
        let piece_type = fourth.piece_type;
        let capture = fourth.capture;
        let disambiguation = fourth.disambiguation;

        let current_pgn_char = self
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("fourth"))?;

        if current_pgn_char.is_digit(10) && (capture || disambiguation.is_some()) {
            let Some(col) = dest_col else {
                return Err(ChessPositionError::MissingDestinationColumn.into());
            };

            let dest_line = current_pgn_char;
            let destination = ChessPosition::new(dest_line, col).try_into()?;
            let origin = self.game_state.find_piece_position(
                piece_type,
                destination,
                disambiguation,
                capture,
            )?;

            self.next_move = Some(Move::new(origin, destination));
        }
        else if current_pgn_char.is_lowercase() {
            dest_col = Some(current_pgn_char);
            self.next_move = None;
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        self.state = PgnParserState::Fifth(Fifth {
            capture,
            disambiguation,
            dest_col,
            piece_type,
        });

        Ok(())
    }

    fn parse_fifth_char(&mut self, fifth: Fifth) -> Result<(), MoveError> {
        let capture = fifth.capture;
        let disambiguation = fifth.disambiguation;
        let dest_col = fifth.dest_col;
        let piece_type = fifth.piece_type;

        let current_pgn_char = self
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("fifth"))?;

        if !current_pgn_char.is_digit(10) {
            return Err(ChessPositionError::MissingDestinationLine.into());
        }

        let Some(col) = dest_col else {
            return Err(ChessPositionError::MissingDestinationColumn.into());
        };

        let dest_line = current_pgn_char;
        let destination = ChessPosition::new(dest_line, col).try_into()?;
        let origin = self.game_state.find_piece_position(
            piece_type,
            destination,
            disambiguation,
            capture,
        )?;

        self.next_move = Some(Move::new(origin, destination));

        self.state = PgnParserState::Finished;

        Ok(())
    }
}

pub fn parse_move(game_state: &GameState, str_move: String) -> Result<Move, MoveError> {
    let mut pgn_parser = PgnParser::new(game_state, &str_move);

    pgn_parser.step()?;

    while pgn_parser.next_move.is_none() {
        pgn_parser.step()?;
    }

    Ok(pgn_parser.next_move.unwrap())
}
