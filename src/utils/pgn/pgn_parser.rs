use crate::errors::MoveError;
use crate::game_state::GameState;
use crate::utils::constants::QUEEN_SIDE_CASTLING;
use crate::utils::types::Move;

use super::pgn_parser_steps::First;

pub fn parse_move(game_state: &GameState, str_move: &str) -> Result<Move, MoveError> {
    let first = First {
        pgn_len: str_move.len(),
    };

    first.parse(game_state, str_move.chars(), QUEEN_SIDE_CASTLING.chars())
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
