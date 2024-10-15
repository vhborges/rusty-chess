use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::piece::PieceType;
use crate::utils::types::Move;
use crate::utils::ChessPosition;
use crate::utils::constants::INTERNAL_ERROR_03;
use super::super::pgn_utils::{PgnParser, PgnParserState};
use super::Fifth;

const STEP: &str = "fourth";

#[derive(Copy, Clone)]
pub struct Fourth {
    pub capture: bool,
    pub disambiguation: Option<char>,
    pub dest_col: Option<char>,
    pub piece_type: PieceType,
    pub castling: bool,
}

impl Fourth {
    pub fn parse(self, pgn_parser: &mut PgnParser) -> Result<(), MoveError> {
        let mut dest_col = self.dest_col;
        let piece_type = self.piece_type;
        let capture = self.capture;
        let disambiguation = self.disambiguation;

        let current_pgn_char = pgn_parser
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter(STEP))?;

        if self.castling {
            return Self::handle_castling(pgn_parser, piece_type, dest_col, current_pgn_char);
        }
        else if current_pgn_char.is_ascii_digit() && (capture || disambiguation.is_some()) {
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

        pgn_parser.state = PgnParserState::Fifth(Fifth {
            capture,
            disambiguation,
            dest_col,
            piece_type,
            castling: false,
        });

        Ok(())
    }

    fn handle_castling(
        pgn_parser: &mut PgnParser,
        piece_type: PieceType,
        dest_col: Option<char>,
        current_pgn_char: char,
    ) -> Result<(), MoveError> {
        if current_pgn_char == pgn_parser.castling_chars.next().expect(INTERNAL_ERROR_03) {
            pgn_parser.state = PgnParserState::Fifth(Fifth {
                capture: false,
                disambiguation: None,
                dest_col,
                piece_type,
                castling: true,
            });

            Ok(())
        }
        else {
            // TODO implement king size castling
            
        }
    }
}
