use super::Third;
use super::common::{CommonIters, ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::constants::INTERNAL_ERROR_03;
use crate::errors::{MoveError, PgnError};
use crate::pgn::constants::CAPTURE;
use crate::pieces::PieceType;
use crate::types::board::constants::{COL_RANGE, LINE_RANGE};
use crate::types::{ChessPosition, Move};

const STEP: &str = "second";

pub struct Second<'a> {
    pgn_len: usize,
    state: ParserState,
    iters: CommonIters<'a>,
}

impl<'a, 'b> Second<'a>
where
    Self: 'b,
{
    pub fn new(pgn_len: usize, state: ParserState, iters: CommonIters<'a>) -> Box<Self> {
        Box::new(Self {
            pgn_len,
            state,
            iters,
        })
    }

    fn handle_castling(mut self, current_pgn_char: char) -> Result<StepResult<'b>, MoveError> {
        if current_pgn_char == self.iters.castling_chars.next().expect(INTERNAL_ERROR_03) {
            Ok(StepResult::Step(Third::new(self.state, self.iters)))
        }
        else {
            Err(PgnError::MissingCharacter(STEP).into())
        }
    }

    fn handle_digit(
        mut self,
        game_state: &GameState,
        piece_type: PieceType,
        current_pgn_char: char,
    ) -> Result<StepResult<'b>, MoveError> {
        match self.state.dest_col {
            Some(col) => {
                let dest_line = current_pgn_char;
                let destination = ChessPosition::new(dest_line, col).try_into()?;

                let origin =
                    game_state.find_piece_position(piece_type, destination, None, false)?;

                Ok(StepResult::Move(Move::new(origin, destination)))
            }
            None => {
                self.state.disambiguation = Some(current_pgn_char);
                self.state.capture = false;

                Ok(StepResult::Step(Third::new(self.state, self.iters)))
            }
        }
    }

    fn handle_capture(mut self, piece_type: PieceType) -> Result<StepResult<'b>, MoveError> {
        self.state.capture = true;

        if piece_type != PieceType::Pawn {
            self.state.disambiguation = None;
        }

        Ok(StepResult::Step(Third::new(self.state, self.iters)))
    }

    fn handle_disambiguation(
        mut self,
        current_pgn_char: char,
    ) -> Result<StepResult<'b>, MoveError> {
        self.state.disambiguation = Some(current_pgn_char);
        self.state.capture = false;

        Ok(StepResult::Step(Third::new(self.state, self.iters)))
    }

    fn handle_destination_column(
        mut self,
        current_pgn_char: char,
    ) -> Result<StepResult<'b>, MoveError> {
        self.state.dest_col = Some(current_pgn_char);
        self.state.disambiguation = None;
        self.state.capture = false;

        Ok(StepResult::Step(Third::new(self.state, self.iters)))
    }
}

impl PgnParserStep for Second<'_> {
    fn parse<'a>(mut self: Box<Self>, game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let piece_type = self.state.piece_type;

        let current_pgn_char = self
            .iters
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter(STEP))?;

        if self.state.castling {
            self.handle_castling(current_pgn_char)
        }
        else if current_pgn_char.is_ascii_digit() {
            self.handle_digit(game_state, piece_type, current_pgn_char)
        }
        else if current_pgn_char == CAPTURE {
            self.handle_capture(piece_type)
        }
        else if self.pgn_len > 3
            && piece_type != PieceType::Pawn
            && (LINE_RANGE.contains(&current_pgn_char) || COL_RANGE.contains(&current_pgn_char))
        {
            self.handle_disambiguation(current_pgn_char)
        }
        else if current_pgn_char.is_lowercase() {
            self.handle_destination_column(current_pgn_char)
        }
        else {
            Err(PgnError::InvalidCharacter(current_pgn_char).into())
        }
    }
}
