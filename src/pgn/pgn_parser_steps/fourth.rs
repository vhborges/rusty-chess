use super::Fifth;
use super::common::{CommonIters, ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::constants::INTERNAL_ERROR_03;
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::types::{ChessPosition, Move};

const STEP: &str = "fourth";

pub struct Fourth<'a> {
    state: ParserState,
    iters: CommonIters<'a>,
}

impl<'a, 'b> Fourth<'a>
where
    Self: 'b,
{
    pub fn new(state: ParserState, iters: CommonIters<'a>) -> Box<Self> {
        Box::new(Self { state, iters })
    }

    fn handle_castling(
        mut self,
        current_pgn_char: Option<char>,
        game_state: &GameState,
    ) -> Result<StepResult<'b>, MoveError> {
        match current_pgn_char {
            Some(pgn_char) => {
                if pgn_char == self.iters.castling_chars.next().expect(INTERNAL_ERROR_03) {
                    Ok(StepResult::Step(Fifth::new(self.state, self.iters)))
                }
                else {
                    Err(PgnError::InvalidCharacter(pgn_char).into())
                }
            }
            None => game_state.find_castling_move(true).map(|m| m.into()),
        }
    }

    fn handle_digit(
        self,
        game_state: &GameState,
        capture: bool,
        disambiguation: Option<char>,
        current_pgn_char: char,
    ) -> Result<StepResult<'b>, MoveError> {
        let piece_type = self.state.piece_type;

        let Some(col) = self.state.dest_col
        else {
            return Err(ChessPositionError::MissingDestinationColumn.into());
        };

        let dest_line = current_pgn_char;
        let destination = ChessPosition::new(dest_line, col).try_into()?;
        let origin =
            game_state.find_piece_position(piece_type, destination, disambiguation, capture)?;

        Ok(StepResult::Move(Move::new(origin, destination)))
    }

    fn handle_destination_column(
        mut self,
        current_pgn_char: char,
    ) -> Result<StepResult<'b>, MoveError> {
        self.state.dest_col = Some(current_pgn_char);

        Ok(StepResult::Step(Fifth::new(self.state, self.iters)))
    }
}

impl PgnParserStep for Fourth<'_> {
    fn parse<'a>(mut self: Box<Self>, game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let capture = self.state.capture;
        let disambiguation = self.state.disambiguation;
        let castling = self.state.castling;

        let current_pgn_char = self.iters.pgn_chars.next();

        if castling {
            return self.handle_castling(current_pgn_char, game_state);
        }

        let current_pgn_char = current_pgn_char.ok_or(PgnError::MissingCharacter(STEP))?;

        if current_pgn_char.is_ascii_digit() && (capture || disambiguation.is_some()) {
            self.handle_digit(game_state, capture, disambiguation, current_pgn_char)
        }
        else if current_pgn_char.is_lowercase() {
            self.handle_destination_column(current_pgn_char)
        }
        else {
            Err(PgnError::InvalidCharacter(current_pgn_char).into())
        }
    }
}
