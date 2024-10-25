use super::super::pgn_utils::{PgnParser, PgnParserState};
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::piece::PieceType;
use crate::utils::constants::INTERNAL_ERROR_03;
use crate::utils::types::Move;
use crate::utils::ChessPosition;

#[derive(Copy, Clone)]
pub struct Fifth {
    pub capture: bool,
    pub disambiguation: Option<char>,
    pub dest_col: Option<char>,
    pub piece_type: PieceType,
    pub castling: bool,
}

impl Fifth {
    pub fn parse(self, pgn_parser: &mut PgnParser) -> Result<(), MoveError> {
        let capture = self.capture;
        let disambiguation = self.disambiguation;
        let dest_col = self.dest_col;
        let piece_type = self.piece_type;

        let current_pgn_char = pgn_parser
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter("fifth"))?;

        if self.castling {
            return Self::handle_castling(pgn_parser, current_pgn_char);
        }
        else if !current_pgn_char.is_ascii_digit() {
            return Err(ChessPositionError::MissingDestinationLine.into());
        }

        let Some(col) = dest_col
        else {
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

        pgn_parser.state = PgnParserState::Finished;

        Ok(())
    }

    fn handle_castling(
        pgn_parser: &mut PgnParser,
        current_pgn_char: char,
    ) -> Result<(), MoveError> {
        if current_pgn_char == pgn_parser.castling_chars.next().expect(INTERNAL_ERROR_03) {
            let (king_move, rook_move) = pgn_parser.game_state.find_castling_move(false)?;
            pgn_parser.next_move = Some(king_move);
            pgn_parser.additional_next_move = Some(rook_move);

            pgn_parser.state = PgnParserState::Finished;

            Ok(())
        }
        else {
            Err(PgnError::InvalidCharacter(current_pgn_char).into())
        }
    }
}
