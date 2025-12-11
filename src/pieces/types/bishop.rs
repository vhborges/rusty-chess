use crate::Board;
use crate::movement::{Direction, Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub fn can_move(board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let nr_of_squares = (dest.col - src.col).unsigned_abs() as usize;

    is_move_valid(src, dest) && board.is_path_clear(src, dest, nr_of_squares)
}

pub fn attacks(board: &Board, source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let nr_of_squares = ((dest.col - src.col).abs() - 1) as usize;

    is_move_valid(src, dest) && board.is_path_clear(src, dest, nr_of_squares)
}

pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
    let mut result = Vec::new();
    let pos_i8 = source.into();

    for dx in (-1..=1).step_by(2) {
        for dy in (-1..=1).step_by(2) {
            let direction = Direction::new(dx, dy, pos_i8);

            for pos in direction {
                if board.is_position_occupied(pos) {
                    break;
                }
                result.push(pos);
            }
        }
    }

    result
}

fn is_move_valid(source: PositionI8, destination: PositionI8) -> bool {
    if (source.line == destination.line) || (source.col == destination.col) {
        return false;
    }

    if (source.line - destination.line).abs() != (source.col - destination.col).abs() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::setup_board;

    #[test]
    fn test_get_possible_moves_empty_board_bottom_left() {
        let board = setup_board(Some("tests/bishop/only_bishop_bottom_left.txt"));
        let source = Position::new(7, 0);

        let expected = vec![
            Position::new(6, 1),
            Position::new(5, 2),
            Position::new(4, 3),
            Position::new(3, 4),
            Position::new(2, 5),
            Position::new(1, 6),
            Position::new(0, 7),
        ];

        let result = get_possible_moves(&board, source);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_possible_moves_empty_board_top_right() {
        let board = setup_board(Some("tests/bishop/only_bishop_top_right.txt"));
        let source = Position::new(0, 7);

        let expected = vec![
            Position::new(1, 6),
            Position::new(2, 5),
            Position::new(3, 4),
            Position::new(4, 3),
            Position::new(5, 2),
            Position::new(6, 1),
            Position::new(7, 0),
        ];

        let result = get_possible_moves(&board, source);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_possible_moves_empty_board_top_left() {
        let board = setup_board(Some("tests/bishop/only_bishop_top_left.txt"));
        let source = Position::new(0, 0);

        let expected = vec![
            Position::new(1, 1),
            Position::new(2, 2),
            Position::new(3, 3),
            Position::new(4, 4),
            Position::new(5, 5),
            Position::new(6, 6),
            Position::new(7, 7),
        ];

        let result = get_possible_moves(&board, source);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_possible_moves_empty_board_bottom_right() {
        let board = setup_board(Some("tests/bishop/only_bishop_bottom_right.txt"));
        let source = Position::new(7, 7);

        let expected = vec![
            Position::new(6, 6),
            Position::new(5, 5),
            Position::new(4, 4),
            Position::new(3, 3),
            Position::new(2, 2),
            Position::new(1, 1),
            Position::new(0, 0),
        ];

        let result = get_possible_moves(&board, source);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_possible_moves_initial_position() {
        let board = setup_board(None);
        let source = Position::new(7, 2);

        let expected = Vec::new();

        let result = get_possible_moves(&board, source);

        assert_eq!(result, expected)
    }
}
