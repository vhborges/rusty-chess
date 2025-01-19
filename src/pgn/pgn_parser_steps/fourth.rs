use super::Fifth;
use super::common::{ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::types::{ChessPosition, Move};
use crate::utils::constants::INTERNAL_ERROR_03;

const STEP: &str = "fourth";

pub struct Fourth<'a> {
    pub state: ParserState<'a>,
}

impl PgnParserStep for Fourth<'_> {
    fn parse<'a>(mut self: Box<Self>, game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let piece_type = self.state.piece_type;
        let capture = self.state.capture;
        let disambiguation = self.state.disambiguation;
        let castling = self.state.castling;

        let current_pgn_char = self.state.pgn_chars.next();

        if castling {
            return self.handle_castling(current_pgn_char, game_state);
        }

        let current_pgn_char = current_pgn_char.ok_or(PgnError::MissingCharacter(STEP))?;

        if current_pgn_char.is_ascii_digit() && (capture || disambiguation.is_some()) {
            let Some(col) = self.state.dest_col
            else {
                return Err(ChessPositionError::MissingDestinationColumn.into());
            };

            let dest_line = current_pgn_char;
            let destination = ChessPosition::new(dest_line, col).try_into()?;
            let origin =
                game_state.find_piece_position(piece_type, destination, disambiguation, capture)?;

            return Ok(StepResult::Move(Move::new(origin, destination)));
        }
        else if current_pgn_char.is_lowercase() {
            self.state.dest_col = Some(current_pgn_char);
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        let fifth = Fifth { state: self.state };

        Ok(StepResult::Step(Box::new(fifth)))
    }
}

impl Fourth<'_> {
    fn handle_castling<'a>(
        mut self,
        current_pgn_char: Option<char>,
        game_state: &GameState,
    ) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        match current_pgn_char {
            Some(pgn_char) => {
                if pgn_char == self.state.castling_chars.next().expect(INTERNAL_ERROR_03) {
                    let fifth = Fifth { state: self.state };

                    Ok(StepResult::Step(Box::new(fifth)))
                }
                else {
                    Err(PgnError::InvalidCharacter(pgn_char).into())
                }
            }
            None => game_state.find_castling_move(true).map(|m| m.into()),
        }
    }
}
