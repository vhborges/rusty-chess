use super::Second;
use crate::GameState;
use crate::errors::{MoveError, PgnError};
use crate::pgn::pgn_parser_steps::common::ParserState;
use crate::piece::PieceType;
use crate::utils::constants::INTERNAL_ERROR_03;
use crate::utils::types::Move;
use std::str::Chars;

pub struct First {
    pub pgn_len: usize,
}

impl First {
    pub fn parse(
        self,
        game_state: &GameState,
        mut pgn_chars: Chars,
        mut castling_chars: Chars,
    ) -> Result<Move, MoveError> {
        let current_pgn_char = pgn_chars.next().ok_or(PgnError::EmptyInput)?;
        let piece_type: PieceType = current_pgn_char.try_into()?;

        let dest_col: Option<char> = if piece_type == PieceType::Pawn {
            Some(current_pgn_char)
        }
        else {
            None
        };

        let castling = current_pgn_char == castling_chars.next().expect(INTERNAL_ERROR_03);

        let step = Second {
            pgn_len: self.pgn_len,
            pgn_first_char: current_pgn_char,
            state: ParserState {
                piece_type,
                capture: false,
                castling,
                dest_col,
                disambiguation: None,
            },
        };

        step.parse(game_state, pgn_chars, castling_chars)
    }
}
