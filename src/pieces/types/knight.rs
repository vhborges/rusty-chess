use crate::Board;
use crate::movement::{Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2658}', '\u{265E}'];

pub fn can_move(source: Position, destination: Position) -> bool {
    let src: PositionI8 = source.into();
    let dest: PositionI8 = destination.into();

    let vertical_distance = (src.line - dest.line).abs();
    let horizontal_distance = (src.col - dest.col).abs();

    (vertical_distance == 1 && horizontal_distance == 2)
        || (vertical_distance == 2 && horizontal_distance == 1)
}

pub fn attacks(origin: Position, destination: Position) -> bool {
    can_move(origin, destination)
}

pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
    let mut result = Vec::new();

    for dx in (-2..=2).step_by(4) {
        for dy in (-1..=1).step_by(2) {
            let dest_i8 = PositionI8::new(source.line as i8 + dy, source.col as i8 + dx);
            if let Ok(dest) = dest_i8.try_into()
                && !board.is_position_occupied(dest)
            {
                result.push(dest);
            }
        }
    }

    for dx in (-1..=1).step_by(2) {
        for dy in (-2..=2).step_by(4) {
            let dest_i8 = PositionI8::new(source.line as i8 + dy, source.col as i8 + dx);
            if let Ok(dest) = dest_i8.try_into()
                && !board.is_position_occupied(dest)
            {
                result.push(dest);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::types::pawn::Pawn;
    use crate::pieces::{Color, Piece, PieceType};
    use crate::utils::test_helper::setup_board;

    #[test]
    fn test_can_move_all_valid_l_shapes() {
        let source = Position::new(4, 4);

        assert!(can_move(source, Position::new(2, 3)));
        assert!(can_move(source, Position::new(2, 5)));
        assert!(can_move(source, Position::new(3, 2)));
        assert!(can_move(source, Position::new(3, 6)));
        assert!(can_move(source, Position::new(5, 2)));
        assert!(can_move(source, Position::new(5, 6)));
        assert!(can_move(source, Position::new(6, 3)));
        assert!(can_move(source, Position::new(6, 5)));
    }

    #[test]
    fn test_can_move_invalid_moves() {
        let source = Position::new(4, 4);

        assert!(!can_move(source, Position::new(4, 4)));
        assert!(!can_move(source, Position::new(4, 5)));
        assert!(!can_move(source, Position::new(5, 4)));
        assert!(!can_move(source, Position::new(5, 5)));
        assert!(!can_move(source, Position::new(4, 6)));
        assert!(!can_move(source, Position::new(6, 4)));
        assert!(!can_move(source, Position::new(6, 6)));
    }

    #[test]
    fn test_can_move_from_corner() {
        let source = Position::new(7, 0);

        assert!(can_move(source, Position::new(5, 1)));
        assert!(can_move(source, Position::new(6, 2)));
        assert!(!can_move(source, Position::new(7, 1)));
    }

    #[test]
    fn test_attacks_parity_with_can_move() {
        let source = Position::new(4, 4);

        assert_eq!(
            attacks(source, Position::new(2, 3)),
            can_move(source, Position::new(2, 3))
        );
        assert_eq!(
            attacks(source, Position::new(6, 5)),
            can_move(source, Position::new(6, 5))
        );
        assert_eq!(
            attacks(source, Position::new(4, 5)),
            can_move(source, Position::new(4, 5))
        );
    }

    #[test]
    fn test_get_possible_moves_from_corner() {
        let board = setup_board(Some("tests/knight/only_knight_corner.txt"));
        let source = Position::new(7, 0);

        let result = get_possible_moves(&board, source);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&Position::new(5, 1)));
        assert!(result.contains(&Position::new(6, 2)));
    }

    #[test]
    fn test_get_possible_moves_from_edge() {
        let board = setup_board(Some("tests/knight/only_knight_edge.txt"));
        let source = Position::new(4, 0);

        let result = get_possible_moves(&board, source);

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_get_possible_moves_from_center() {
        let board = setup_board(Some("tests/knight/only_knight_center.txt"));
        let source = Position::new(4, 4);

        let result = get_possible_moves(&board, source);

        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_get_possible_moves_with_blocked_squares() {
        let mut board = setup_board(Some("tests/knight/only_knight_center.txt"));
        let source = Position::new(4, 4);

        board.add_piece(
            Piece::new(PieceType::Pawn(Pawn::new()), Color::White),
            Position::new(2, 3),
        );
        board.add_piece(
            Piece::new(PieceType::Pawn(Pawn::new()), Color::White),
            Position::new(3, 6),
        );

        let result = get_possible_moves(&board, source);

        assert_eq!(result.len(), 6);
        assert!(!result.contains(&Position::new(2, 3)));
        assert!(!result.contains(&Position::new(3, 6)));
    }
}
