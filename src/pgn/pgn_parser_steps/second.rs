use super::Third;
use super::common::{ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::{MoveError, PgnError};
use crate::piece::PieceType;
use crate::utils::constants::{CAPTURE, COL_RANGE, INTERNAL_ERROR_03, LINE_RANGE};
use crate::utils::{ChessPosition, Move};
use std::str::Chars;

const STEP: &str = "second";

pub struct Second<'a> {
    pub pgn_len: usize,
    pub pgn_first_char: char,
    pub castling: bool,
    pub piece_type: PieceType,
    pub dest_col: Option<char>,
    pub pgn_chars: Chars<'a>,
    pub castling_chars: Chars<'static>,
}

impl PgnParserStep for Second<'_> {
    // TODO create functions for each condition, store pgn state in each function and return a Result
    fn parse<'a>(mut self: Box<Self>, game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let disambiguation;
        let capture;

        let piece_type = self.piece_type;

        let current_pgn_char = self
            .pgn_chars
            .next()
            .ok_or(PgnError::MissingCharacter(STEP))?;

        if self.castling {
            return self.handle_castling(current_pgn_char);
        }
        else if current_pgn_char.is_ascii_digit() {
            match self.dest_col {
                Some(col) => {
                    let dest_line = current_pgn_char;
                    let destination = ChessPosition::new(dest_line, col).try_into()?;

                    let origin =
                        game_state.find_piece_position(piece_type, destination, None, false)?;

                    return Ok(StepResult::Move(Move::new(origin, destination)));
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
            self.dest_col = Some(current_pgn_char);
            disambiguation = None;
            capture = false;
        }
        else {
            return Err(PgnError::InvalidCharacter(current_pgn_char).into());
        }

        let third = Third {
            state: ParserState {
                piece_type: self.piece_type,
                capture,
                castling: self.castling,
                dest_col: self.dest_col,
                disambiguation,
                pgn_chars: self.pgn_chars,
                castling_chars: self.castling_chars,
            },
        };

        Ok(StepResult::Step(Box::new(third)))
    }
}

impl Second<'_> {
    fn handle_castling<'a>(mut self, current_pgn_char: char) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        if current_pgn_char == self.castling_chars.next().expect(INTERNAL_ERROR_03) {
            let third = Third {
                state: ParserState {
                    piece_type: self.piece_type,
                    capture: false,
                    castling: self.castling,
                    dest_col: self.dest_col,
                    disambiguation: None,
                    pgn_chars: self.pgn_chars,
                    castling_chars: self.castling_chars,
                },
            };

            Ok(StepResult::Step(Box::new(third)))
        }
        else {
            Err(PgnError::MissingCharacter(STEP).into())
        }
    }
}
