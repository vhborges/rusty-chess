use super::Fourth;
use super::common::{CommonIters, ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::piece::PieceType;
use crate::types::{ChessPosition, Move};
use crate::utils::constants::{CAPTURE, INTERNAL_ERROR_03};

const STEP: &str = "third";

pub struct Third<'a> {
    state: ParserState,
    iters: CommonIters<'a>,
}

impl<'a> Third<'a> {
    pub fn new(state: ParserState, iters: CommonIters<'a>) -> Box<Self> {
        Box::new(Self { state, iters })
    }

    fn handle_castling<'b>(mut self, current_pgn_char: char) -> Result<StepResult<'b>, MoveError>
    where
        Self: 'b,
    {
        if current_pgn_char == self.iters.castling_chars.next().expect(INTERNAL_ERROR_03) {
            Ok(StepResult::Step(Fourth::new(self.state, self.iters)))
        }
        else {
            Err(PgnError::MissingCharacter(STEP).into())
        }
    }
}

impl PgnParserStep for Third<'_> {
    fn parse<'a>(mut self: Box<Self>, game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let piece_type = self.state.piece_type;
        let disambiguation = self.state.disambiguation;
        let castling = self.state.castling;

        let current_pgn_char = self
            .iters
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter(STEP))?;

        if castling {
            return self.handle_castling(current_pgn_char);
        }
        else if current_pgn_char == CAPTURE {
            self.state.capture = true;
        }
        else if current_pgn_char.is_ascii_digit() && piece_type != PieceType::Pawn {
            let Some(col) = self.state.dest_col
            else {
                return Err(ChessPositionError::MissingDestinationColumn.into());
            };

            let dest_line = current_pgn_char;
            let destination = ChessPosition::new(dest_line, col).try_into()?;
            let origin = game_state.find_piece_position(
                piece_type,
                destination,
                disambiguation,
                self.state.capture,
            )?;

            return Ok(StepResult::Move(Move::new(origin, destination)));
        }
        else if current_pgn_char.is_lowercase() {
            self.state.dest_col = Some(current_pgn_char);
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        Ok(StepResult::Step(Fourth::new(self.state, self.iters)))
    }
}
