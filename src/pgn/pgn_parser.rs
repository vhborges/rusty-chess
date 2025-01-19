use super::pgn_parser_steps::First;
use super::pgn_parser_steps::common::{PgnParserStep, StepResult};
use crate::errors::MoveError;
use crate::game_state::GameState;
use crate::utils::Move;
use crate::utils::constants::{INTERNAL_ERROR_05, QUEEN_SIDE_CASTLING};

pub fn parse_move(game_state: &GameState, str_move: &str) -> Result<Move, MoveError> {
    let first_step = Box::new(First {
        pgn_len: str_move.len(),
        pgn_chars: str_move.chars(),
        castling_chars: QUEEN_SIDE_CASTLING.chars(),
    });

    let mut result = first_step.parse(game_state)?;
    while let StepResult::Step(next_step) = result {
        result = next_step.parse(game_state)?;
    }

    match result {
        StepResult::Move(move_) => Ok(move_),
        _ => panic!("{}", INTERNAL_ERROR_05),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::setup;

    #[test]
    fn test_disambiguation_same_column() -> Result<(), MoveError> {
        let game_state = setup(Some("tests/validate_disambiguation_same_column.txt"));

        let result = parse_move(&game_state, "N3d4")?;

        assert_eq!(result.source().line, 5);

        Ok(())
    }

    #[test]
    fn test_disambiguation_same_line() -> Result<(), MoveError> {
        let game_state = setup(Some("tests/validate_disambiguation_same_line.txt"));

        let result = parse_move(&game_state, "Ncd5")?;

        assert_eq!(result.source().col, 2);

        Ok(())
    }
}
