use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::pieces::PieceType;
use crate::utils::constants::CAPTURE;
use crate::utils::types::Move;
use crate::utils::ChessPosition;

use super::super::pgn_utils::{PgnParser, PgnParserState};
use super::Fourth;

#[derive(Copy, Clone)]
pub struct Third {
    pub capture: bool,
    pub disambiguation: Option<char>,
    pub dest_col: Option<char>,
    pub piece_type: PieceType,
}

impl Third {
    pub fn parse(self, pgn_parser: &mut PgnParser) -> Result<(), MoveError> {
        let piece_type = self.piece_type;
        let mut dest_col = self.dest_col;
        let mut capture = self.capture;
        let disambiguation = self.disambiguation;

        let current_pgn_char = pgn_parser
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("third"))?;

        if current_pgn_char == CAPTURE {
            capture = true;
            pgn_parser.next_move = None;
        }
        else if current_pgn_char.is_digit(10) && piece_type != PieceType::Pawn {
            let Some(col) = dest_col else {
                return Err(ChessPositionError::MissingDestinationColumn.into());
            };

            let dest_line = current_pgn_char;
            let destination = ChessPosition::new(dest_line, col).try_into()?;
            let origin = pgn_parser.game_state.find_piece_position(
                piece_type,
                destination,
                disambiguation,
                capture,
            )?;

            pgn_parser.next_move = Some(Move::new(origin, destination));
        }
        else if current_pgn_char.is_lowercase() {
            dest_col = Some(current_pgn_char);
            pgn_parser.next_move = None;
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        pgn_parser.state = PgnParserState::Fourth(Fourth {
            capture,
            disambiguation,
            dest_col,
            piece_type,
        });

        Ok(())
    }
}
