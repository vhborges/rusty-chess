use super::Fourth;
use super::common::{ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::piece::PieceType;
use crate::utils::constants::{CAPTURE, INTERNAL_ERROR_03};
use crate::utils::{ChessPosition, Move};

const STEP: &str = "third";

pub struct Third<'a> {
    pub state: ParserState<'a>,
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
            .state
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

        let fourth = Fourth { state: self.state };

        Ok(StepResult::Step(Box::new(fourth)))
    }
}

impl Third<'_> {
    fn handle_castling<'a>(mut self, current_pgn_char: char) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        if current_pgn_char == self.state.castling_chars.next().expect(INTERNAL_ERROR_03) {
            let fourth = Fourth { state: self.state };

            Ok(StepResult::Step(Box::new(fourth)))
        }
        else {
            Err(PgnError::MissingCharacter(STEP).into())
        }
    }
}
