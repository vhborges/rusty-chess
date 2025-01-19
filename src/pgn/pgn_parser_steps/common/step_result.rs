use super::PgnParserStep;
use crate::utils::Move;

/// Represents the result of a single step parse.
/// Can be a Move (if the parse is complete) or the next Step.
pub enum StepResult<'a> {
    Step(Box<dyn PgnParserStep + 'a>),
    Move(Move),
}

impl From<Move> for StepResult<'_> {
    fn from(move_: Move) -> Self {
        StepResult::Move(move_)
    }
}
