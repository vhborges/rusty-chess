use super::Fourth;
use crate::GameState;
use crate::errors::{ChessPositionError, MoveError, PgnError};
use crate::pgn::pgn_parser_steps::common::ParserState;
use crate::piece::PieceType;
use crate::utils::{ChessPosition, Move};
use crate::utils::constants::{CAPTURE, INTERNAL_ERROR_03};
use std::str::Chars;

const STEP: &str = "third";

pub struct Third {
    pub state: ParserState,
}

impl Third {
    pub fn parse(
        self,
        game_state: &GameState,
        mut pgn_chars: Chars,
        castling_chars: Chars,
    ) -> Result<Move, MoveError> {
        let piece_type = self.state.piece_type;
        let mut dest_col = self.state.dest_col;
        let mut capture = self.state.capture;
        let disambiguation = self.state.disambiguation;
        let castling = self.state.castling;

        let current_pgn_char = pgn_chars.next().ok_or(PgnError::MissingCharacter(STEP))?;

        if castling {
            return self.handle_castling(current_pgn_char, castling_chars, game_state, pgn_chars);
        }
        else if current_pgn_char == CAPTURE {
            capture = true;
        }
        else if current_pgn_char.is_ascii_digit() && piece_type != PieceType::Pawn {
            let Some(col) = dest_col
            else {
                return Err(ChessPositionError::MissingDestinationColumn.into());
            };

            let dest_line = current_pgn_char;
            let destination = ChessPosition::new(dest_line, col).try_into()?;
            let origin =
                game_state.find_piece_position(piece_type, destination, disambiguation, capture)?;

            return Ok(Move::new(origin, destination));
        }
        else if current_pgn_char.is_lowercase() {
            dest_col = Some(current_pgn_char);
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        let step = Fourth {
            state: ParserState {
                piece_type,
                capture,
                castling,
                dest_col,
                disambiguation,
            },
        };

        step.parse(game_state, pgn_chars, castling_chars)
    }

    fn handle_castling(
        self,
        current_pgn_char: char,
        mut castling_chars: Chars,
        game_state: &GameState,
        pgn_chars: Chars,
    ) -> Result<Move, MoveError> {
        if current_pgn_char == castling_chars.next().expect(INTERNAL_ERROR_03) {
            let step = Fourth {
                state: ParserState {
                    piece_type: self.state.piece_type,
                    capture: self.state.capture,
                    castling: true,
                    dest_col: self.state.dest_col,
                    disambiguation: self.state.disambiguation,
                },
            };

            step.parse(game_state, pgn_chars, castling_chars)
        }
        else {
            Err(PgnError::MissingCharacter(STEP).into())
        }
    }
}
