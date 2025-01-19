use super::StepResult;
use crate::GameState;
use crate::errors::MoveError;

/// Defines common behaviour for all steps
pub trait PgnParserStep {
    fn parse<'a>(self: Box<Self>, game_state: &GameState) -> Result<StepResult<'a>, MoveError>
    where
        Self: 'a;
}
