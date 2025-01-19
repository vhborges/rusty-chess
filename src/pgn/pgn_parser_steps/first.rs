use super::Second;
use super::common::{PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::{MoveError, PgnError};
use crate::piece::PieceType;
use crate::utils::constants::INTERNAL_ERROR_03;
use std::str::Chars;

pub struct First<'a> {
    pub pgn_len: usize,
    pub pgn_chars: Chars<'a>,
    pub castling_chars: Chars<'static>,
}

impl PgnParserStep for First<'_> {
    fn parse<'a>(mut self: Box<Self>, _game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let current_pgn_char = self.pgn_chars.next().ok_or(PgnError::EmptyInput)?;
        let piece_type: PieceType = current_pgn_char.try_into()?;

        let dest_col: Option<char> = if piece_type == PieceType::Pawn {
            Some(current_pgn_char)
        }
        else {
            None
        };

        let castling = current_pgn_char == self.castling_chars.next().expect(INTERNAL_ERROR_03);

        let second = Second {
            pgn_len: self.pgn_len,
            pgn_first_char: current_pgn_char,
            piece_type,
            castling,
            dest_col,
            pgn_chars: self.pgn_chars,
            castling_chars: self.castling_chars,
        };

        Ok(StepResult::Step(Box::new(second)))
    }
}
