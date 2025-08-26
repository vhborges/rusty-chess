use super::Fourth;
use super::common::{CommonIters, ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::constants::INTERNAL_ERROR_03;
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::pgn::constants::CAPTURE;
use crate::pieces::PieceType;
use crate::types::{ChessPosition, Move};

const STEP: &str = "third";

pub struct Third<'a> {
    state: ParserState,
    iters: CommonIters<'a>,
}

impl<'a, 'b> Third<'a>
where
    Self: 'b,
{
    pub fn new(state: ParserState, iters: CommonIters<'a>) -> Box<Self> {
        Box::new(Self { state, iters })
    }

    fn handle_castling(mut self, current_pgn_char: char) -> Result<StepResult<'b>, MoveError> {
        if current_pgn_char == self.iters.castling_chars.next().expect(INTERNAL_ERROR_03) {
            Ok(StepResult::Step(Fourth::new(self.state, self.iters)))
        }
        else {
            Err(PgnError::MissingCharacter(STEP).into())
        }
    }

    fn handle_capture(mut self) -> Result<StepResult<'b>, MoveError> {
        self.state.capture = true;

        Ok(StepResult::Step(Fourth::new(self.state, self.iters)))
    }

    fn handle_digit(
        self,
        game_state: &GameState,
        piece_type: PieceType,
        current_pgn_char: char,
    ) -> Result<StepResult<'b>, MoveError> {
        let Some(col) = self.state.dest_col
        else {
            return Err(ChessPositionError::MissingDestinationColumn.into());
        };

        let dest_line = current_pgn_char;
        let destination = ChessPosition::new(dest_line, col).try_into()?;
        let origin = game_state.find_piece_position(
            piece_type,
            destination,
            self.state.disambiguation,
            self.state.capture,
        )?;

        Ok(StepResult::Move(Move::new(origin, destination)))
    }

    fn handle_destination_column(
        mut self,
        current_pgn_char: char,
    ) -> Result<StepResult<'b>, MoveError> {
        self.state.dest_col = Some(current_pgn_char);

        Ok(StepResult::Step(Fourth::new(self.state, self.iters)))
    }
}

impl PgnParserStep for Third<'_> {
    fn parse<'a>(mut self: Box<Self>, game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let piece_type = self.state.piece_type;
        let castling = self.state.castling;

        let current_pgn_char = self
            .iters
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter(STEP))?;

        if castling {
            self.handle_castling(current_pgn_char)
        }
        else if current_pgn_char == CAPTURE {
            self.handle_capture()
        }
        else if current_pgn_char.is_ascii_digit() && piece_type != PieceType::Pawn {
            self.handle_digit(game_state, piece_type, current_pgn_char)
        }
        else if current_pgn_char.is_lowercase() {
            self.handle_destination_column(current_pgn_char)
        }
        else {
            Err(PgnError::InvalidCharacter(current_pgn_char).into())
        }
    }
}
