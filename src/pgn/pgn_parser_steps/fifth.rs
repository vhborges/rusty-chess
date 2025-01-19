use super::common::{ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::utils::constants::INTERNAL_ERROR_03;
use crate::utils::{ChessPosition, Move};

pub struct Fifth<'a> {
    pub state: ParserState<'a>,
}

impl PgnParserStep for Fifth<'_> {
    fn parse<'a>(mut self: Box<Self>, game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let capture = self.state.capture;
        let disambiguation = self.state.disambiguation;
        let piece_type = self.state.piece_type;

        let current_pgn_char = self
            .state
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("fifth"))?;

        if self.state.castling {
            return self.handle_castling(current_pgn_char, game_state);
        }
        else if !current_pgn_char.is_ascii_digit() {
            return Err(ChessPositionError::MissingDestinationLine.into());
        }

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
}

impl Fifth<'_> {
    fn handle_castling<'a>(
        &mut self,
        current_pgn_char: char,
        game_state: &GameState,
    ) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        if current_pgn_char == self.state.castling_chars.next().expect(INTERNAL_ERROR_03) {
            game_state.find_castling_move(false).map(|m| m.into())
        }
        else {
            Err(PgnError::InvalidCharacter(current_pgn_char).into())
        }
    }
}
