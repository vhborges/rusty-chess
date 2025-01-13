use super::Third;
use crate::GameState;
use crate::errors::{MoveError, PgnError};
use crate::pgn::pgn_parser_steps::common::ParserState;
use crate::piece::PieceType;
use crate::utils::ChessPosition;
use crate::utils::constants::{CAPTURE, COL_RANGE, INTERNAL_ERROR_03, LINE_RANGE};
use crate::utils::types::Move;
use std::str::Chars;

const STEP: &str = "second";

pub struct Second {
    pub pgn_len: usize,
    pub pgn_first_char: char,
    pub state: ParserState,
}

impl Second {
    // TODO create functions for each condition, store pgn state in each function and return a Result
    pub fn parse(
        self,
        game_state: &GameState,
        mut pgn_chars: Chars,
        castling_chars: Chars,
    ) -> Result<Move, MoveError> {
        let disambiguation;
        let capture;
        let piece_type = self.state.piece_type;
        let mut dest_col = self.state.dest_col;

        let current_pgn_char = pgn_chars.next().ok_or(PgnError::MissingCharacter(STEP))?;

        if self.state.castling {
            return self.handle_castling(current_pgn_char, castling_chars, game_state, pgn_chars);
        }
        else if current_pgn_char.is_ascii_digit() {
            match self.state.dest_col {
                Some(col) => {
                    let dest_line = current_pgn_char;
                    let destination = ChessPosition::new(dest_line, col).try_into()?;

                    disambiguation = None;
                    capture = false;

                    let origin = game_state.find_piece_position(
                        piece_type,
                        destination,
                        disambiguation,
                        capture,
                    )?;

                    return Ok(Move::new(origin, destination));
                }
                None => {
                    disambiguation = Some(current_pgn_char);
                    capture = false;
                }
            }
        }
        else if current_pgn_char == CAPTURE {
            capture = true;

            if piece_type == PieceType::Pawn {
                disambiguation = Some(self.pgn_first_char);
            }
            else {
                disambiguation = None;
            }
        }
        else if self.pgn_len > 3
            && piece_type != PieceType::Pawn
            && (LINE_RANGE.contains(&current_pgn_char) || COL_RANGE.contains(&current_pgn_char))
        {
            disambiguation = Some(current_pgn_char);
            capture = false;
        }
        else if current_pgn_char.is_lowercase() {
            dest_col = Some(current_pgn_char);
            disambiguation = None;
            capture = false;
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        let step = Third {
            state: ParserState {
                capture,
                disambiguation,
                dest_col,
                piece_type,
                castling: false,
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
            let step = Third {
                state: ParserState {
                    capture: false,
                    disambiguation: None,
                    dest_col: self.state.dest_col,
                    piece_type: self.state.piece_type,
                    castling: true,
                },
            };

            step.parse(game_state, pgn_chars, castling_chars)
        }
        else {
            Err(PgnError::MissingCharacter(STEP).into())
        }
    }
}
