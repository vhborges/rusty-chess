use crate::GameState;
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::pgn::pgn_parser_steps::common::ParserState;
use crate::utils::constants::INTERNAL_ERROR_03;
use crate::utils::{ChessPosition, Move};
use std::str::Chars;

pub struct Fifth {
    pub state: ParserState,
}

impl Fifth {
    pub fn parse(
        self,
        game_state: &GameState,
        mut pgn_chars: Chars,
        castling_chars: Chars,
    ) -> Result<Move, MoveError> {
        let capture = self.state.capture;
        let disambiguation = self.state.disambiguation;
        let piece_type = self.state.piece_type;

        let current_pgn_char = pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("fifth"))?;

        if self.state.castling {
            return Self::handle_castling(game_state, castling_chars, current_pgn_char);
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

        Ok(Move::new(origin, destination))
    }

    fn handle_castling(
        game_state: &GameState,
        mut castling_chars: Chars,
        current_pgn_char: char,
    ) -> Result<Move, MoveError> {
        if current_pgn_char == castling_chars.next().expect(INTERNAL_ERROR_03) {
            game_state.find_castling_move(false)
        }
        else {
            Err(PgnError::InvalidCharacter(current_pgn_char).into())
        }
    }
}
