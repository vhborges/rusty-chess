use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::piece::PieceType;
use crate::utils::constants::{CAPTURE, COL_RANGE, LINE_RANGE};
use crate::utils::types::Move;
use crate::utils::ChessPosition;

use super::super::pgn_utils::{PgnParser, PgnParserState};
use super::Third;

#[derive(Copy, Clone)]
pub struct Second {
    pub pgn_len: usize,
    pub pgn_first_char: char,
    pub dest_col: Option<char>,
    pub piece_type: PieceType,
}

impl Second {
    pub fn parse(self, pgn_parser: &mut PgnParser) -> Result<(), MoveError> {
        let disambiguation;
        let capture;
        let piece_type = self.piece_type;
        let mut dest_col = self.dest_col;

        let current_pgn_char = pgn_parser
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("second"))?;

        if current_pgn_char.is_ascii_digit() {
            let Some(col) = self.dest_col else {
                return Err(ChessPositionError::MissingDestinationColumn.into());
            };

            let dest_line = current_pgn_char;
            let destination = ChessPosition::new(dest_line, col).try_into()?;

            disambiguation = None;
            capture = false;

            let origin = pgn_parser.game_state.find_piece_position(
                piece_type,
                destination,
                disambiguation,
                capture,
            )?;

            pgn_parser.next_move = Some(Move::new(origin, destination));
        }
        else if current_pgn_char == CAPTURE {
            capture = true;

            if piece_type == PieceType::Pawn {
                disambiguation = Some(self.pgn_first_char);
            }
            else {
                disambiguation = None;
            }

            pgn_parser.next_move = None;
        }
        else if self.pgn_len > 3
            && piece_type != PieceType::Pawn
            && (LINE_RANGE.contains(&current_pgn_char) || COL_RANGE.contains(&current_pgn_char))
        {
            disambiguation = Some(current_pgn_char);
            capture = false;
            pgn_parser.next_move = None;
        }
        else if current_pgn_char.is_lowercase() {
            dest_col = Some(current_pgn_char);
            disambiguation = None;
            capture = false;
            pgn_parser.next_move = None;
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        pgn_parser.state = PgnParserState::Third(Third {
            capture,
            disambiguation,
            dest_col,
            piece_type,
        });

        Ok(())
    }
}
