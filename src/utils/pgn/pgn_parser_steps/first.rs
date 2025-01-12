use super::super::pgn_utils::{PgnParser, PgnParserState};
use super::Second;
use crate::errors::{MoveError, PgnError};
use crate::piece::PieceType;
use crate::utils::constants::INTERNAL_ERROR_03;

#[derive(Copy, Clone)]
pub struct First {
    pub pgn_len: usize,
}

impl First {
    pub fn parse(self, pgn_parser: &mut PgnParser) -> Result<(), MoveError> {
        let current_pgn_char = pgn_parser.pgn_chars.next().ok_or(PgnError::EmptyInput)?;
        let piece_type: PieceType = current_pgn_char.try_into()?;

        let dest_col: Option<char> = if piece_type == PieceType::Pawn {
            Some(current_pgn_char)
        }
        else {
            None
        };

        let castling =
            current_pgn_char == pgn_parser.castling_chars.next().expect(INTERNAL_ERROR_03);

        pgn_parser.state = PgnParserState::Second(Second {
            pgn_len: self.pgn_len,
            pgn_first_char: current_pgn_char,
            dest_col,
            piece_type,
            castling,
        });

        Ok(())
    }
}
