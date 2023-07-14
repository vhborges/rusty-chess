use super::super::pgn_utils::{PgnParser, PgnParserState};
use super::Second;
use crate::errors::{MoveError, PgnError};
use crate::pieces::PieceType;

#[derive(Copy, Clone)]
pub struct First {
    pub pgn_len: usize,
}

impl First {
    pub fn parse(self, pgn_parser: &mut PgnParser) -> Result<(), MoveError> {
        let dest_col;

        let current_pgn_char = pgn_parser.pgn_chars.next().ok_or(PgnError::EmptyInput)?;
        let piece_type: PieceType = current_pgn_char.try_into()?;

        if piece_type == PieceType::Pawn {
            dest_col = Some(current_pgn_char);
        }
        else {
            dest_col = None;
        }

        pgn_parser.state = PgnParserState::Second(Second {
            pgn_len: self.pgn_len,
            pgn_first_char: current_pgn_char,
            dest_col,
            piece_type,
        });

        Ok(())
    }
}
