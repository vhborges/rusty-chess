use super::Second;
use super::common::{CommonIters, ParserState, PgnParserStep, StepResult};
use crate::GameState;
use crate::errors::constants::INTERNAL_ERROR_03;
use crate::errors::{MoveError, PgnError};
use crate::pgn::constants::QUEEN_SIDE_CASTLING;
use crate::pieces::PieceType;

pub struct First<'a> {
    pgn_len: usize,
    iters: CommonIters<'a>,
    state: ParserState,
}

impl<'a> First<'a> {
    pub fn new(pgn_move: &'a str) -> Box<Self> {
        let iters = CommonIters::new(pgn_move.chars(), QUEEN_SIDE_CASTLING.chars());

        Box::new(Self {
            pgn_len: pgn_move.len(),
            iters,
            state: Default::default(),
        })
    }
}

impl PgnParserStep for First<'_> {
    fn parse<'a>(mut self: Box<Self>, _game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a,
    {
        let current_pgn_char = self.iters.pgn_chars.next().ok_or(PgnError::EmptyInput)?;
        let piece_type: PieceType = current_pgn_char.try_into()?;

        self.state.dest_col = match piece_type {
            PieceType::Pawn(_) => Some(current_pgn_char),
            _ => None,
        };

        self.state.disambiguation = self.state.dest_col; //Possibly, could be used by the second step
        self.state.castling =
            current_pgn_char == self.iters.castling_chars.next().expect(INTERNAL_ERROR_03);
        self.state.piece_type = piece_type;

        Ok(StepResult::Step(Second::new(
            self.pgn_len,
            self.state,
            self.iters,
        )))
    }
}
