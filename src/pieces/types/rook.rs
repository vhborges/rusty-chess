use super::super::Color;
use crate::Board;
use crate::board::constants::{BLACK_CASTLING_LINE, WHITE_CASTLING_LINE};
use crate::movement::{Direction, Position, PositionI8};
use std::cmp::max;

pub const SYMBOLS: [char; 2] = ['\u{2656}', '\u{265C}'];

pub const ROOK_SHORT_CASTLING_COLUMN: usize = 5;
pub const ROOK_LONG_CASTLING_COLUMN: usize = 3;
pub const ROOK_SHORT_CASTLING_INITIAL_COLUMN: usize = 7;
pub const ROOK_LONG_CASTLING_INITIAL_COLUMN: usize = 0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rook {
    pub short_castling_available: bool,
    pub long_castling_available: bool,
}

impl Default for Rook {
    fn default() -> Self {
        Self::new()
    }
}

impl Rook {
    pub fn new() -> Self {
        Self {
            short_castling_available: true,
            long_castling_available: true,
        }
    }

    pub fn can_move(board: &Board, source: Position, destination: Position) -> bool {
        let src: PositionI8 = source.into();
        let dest: PositionI8 = destination.into();

        let nr_of_squares = max((dest.col - src.col).abs(), (dest.line - src.line).abs()) as usize;

        Self::can_move_internal(board, src, dest, nr_of_squares)
    }

    pub fn attacks(board: &Board, source: Position, destination: Position) -> bool {
        let src: PositionI8 = source.into();
        let dest: PositionI8 = destination.into();

        let nr_of_squares =
            (max((dest.col - src.col).abs(), (dest.line - src.line).abs()) - 1) as usize;

        Self::can_move_internal(board, src, dest, nr_of_squares)
    }

    pub fn get_possible_moves(board: &Board, source: Position) -> Vec<Position> {
        let mut result = Vec::new();
        let pos_i8 = source.into();

        for direction in (-1..=1).step_by(2) {
            let vertical_dir = Direction::new(0, direction, pos_i8);
            for pos in vertical_dir {
                if board.is_position_occupied(pos) {
                    break;
                }
                result.push(pos);
            }

            let horizontal_dir = Direction::new(direction, 0, pos_i8);
            for pos in horizontal_dir {
                if board.is_position_occupied(pos) {
                    break;
                }
                result.push(pos);
            }
        }

        result
    }

    fn can_move_internal(
        board: &Board,
        source: PositionI8,
        destination: PositionI8,
        nr_of_squares: usize,
    ) -> bool {
        // Logical XNOR
        if (source.line == destination.line) == (source.col == destination.col) {
            return false;
        }

        board.is_path_clear(source, destination, nr_of_squares)
    }

    pub fn can_castle(&self, board: &Board, origin: Position, destination: Position) -> bool {
        if !self.is_valid_castling(origin) {
            return false;
        }

        Self::can_move(board, origin, destination)
    }

    fn is_valid_castling(&self, origin: Position) -> bool {
        if origin.col == ROOK_SHORT_CASTLING_INITIAL_COLUMN {
            if !self.short_castling_available {
                return false;
            }
        }
        else if origin.col == ROOK_LONG_CASTLING_INITIAL_COLUMN {
            if !self.long_castling_available {
                return false;
            }
        }
        else {
            return false;
        }
        true
    }

    pub fn get_castle_move(turn: Color, is_short_castle: bool) -> (Position, Position) {
        let src_line = match turn {
            Color::White => WHITE_CASTLING_LINE,
            Color::Black => BLACK_CASTLING_LINE,
        };
        let dest_line = src_line;

        let (src_col, dest_col) = match is_short_castle {
            true => (
                ROOK_SHORT_CASTLING_INITIAL_COLUMN,
                ROOK_SHORT_CASTLING_COLUMN,
            ),
            false => (ROOK_LONG_CASTLING_INITIAL_COLUMN, ROOK_LONG_CASTLING_COLUMN),
        };

        let origin = Position::new(src_line, src_col);
        let destination = Position::new(dest_line, dest_col);

        (origin, destination)
    }

    pub fn deny_castling_rights(&mut self) {
        self.long_castling_available = false;
        self.short_castling_available = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::Piece;
    use crate::pieces::PieceType;
    use crate::pieces::types::pawn::Pawn;
    use crate::utils::test_helper::setup_board;

    #[test]
    fn test_can_move_horizontal() {
        let board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        assert!(Rook::can_move(&board, source, Position::new(4, 0)));
        assert!(Rook::can_move(&board, source, Position::new(4, 7)));
        assert!(Rook::can_move(&board, source, Position::new(4, 2)));
    }

    #[test]
    fn test_can_move_vertical() {
        let board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        assert!(Rook::can_move(&board, source, Position::new(0, 4)));
        assert!(Rook::can_move(&board, source, Position::new(7, 4)));
        assert!(Rook::can_move(&board, source, Position::new(2, 4)));
    }

    #[test]
    fn test_can_move_diagonal_returns_false() {
        let board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        assert!(!Rook::can_move(&board, source, Position::new(2, 2)));
        assert!(!Rook::can_move(&board, source, Position::new(6, 6)));
        assert!(!Rook::can_move(&board, source, Position::new(3, 5)));
    }

    #[test]
    fn test_can_move_same_square_returns_false() {
        let board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        assert!(!Rook::can_move(&board, source, source));
    }

    #[test]
    fn test_can_move_blocked_path_returns_false() {
        let mut board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        board.add_piece(
            Piece::new(PieceType::Pawn(Pawn::new()), Color::White),
            Position::new(4, 6),
        );

        assert!(!Rook::can_move(&board, source, Position::new(4, 7)));
        assert!(Rook::can_move(&board, source, Position::new(4, 5)));
    }

    #[test]
    fn test_attacks_vs_can_move_difference() {
        let mut board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        board.add_piece(
            Piece::new(PieceType::Pawn(Pawn::new()), Color::Black),
            Position::new(4, 6),
        );

        assert!(!Rook::can_move(&board, source, Position::new(4, 6)));
        assert!(Rook::attacks(&board, source, Position::new(4, 6)));
    }

    #[test]
    fn test_attacks_clear_path() {
        let board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        assert!(Rook::attacks(&board, source, Position::new(4, 6)));
        assert!(Rook::attacks(&board, source, Position::new(2, 4)));
    }

    #[test]
    fn test_get_possible_moves_from_corner() {
        let board = setup_board(Some("tests/rook/only_rook_corner.txt"));
        let source = Position::new(7, 0);

        let result = Rook::get_possible_moves(&board, source);

        assert_eq!(result.len(), 14);
    }

    #[test]
    fn test_get_possible_moves_from_center() {
        let board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        let result = Rook::get_possible_moves(&board, source);

        assert_eq!(result.len(), 14);
    }

    #[test]
    fn test_get_possible_moves_with_blockers() {
        let mut board = setup_board(Some("tests/rook/only_rook_center.txt"));
        let source = Position::new(4, 4);

        board.add_piece(
            Piece::new(PieceType::Pawn(Pawn::new()), Color::White),
            Position::new(4, 5),
        );
        board.add_piece(
            Piece::new(PieceType::Pawn(Pawn::new()), Color::White),
            Position::new(3, 4),
        );

        let result = Rook::get_possible_moves(&board, source);

        assert_eq!(result.len(), 7);
        assert!(!result.contains(&Position::new(4, 5)));
        assert!(!result.contains(&Position::new(3, 4)));
    }

    #[test]
    fn test_is_valid_castling_short_rook() {
        let rook = Rook::new();
        let origin = Position::new(7, ROOK_SHORT_CASTLING_INITIAL_COLUMN);

        assert!(rook.is_valid_castling(origin));
    }

    #[test]
    fn test_is_valid_castling_long_rook() {
        let rook = Rook::new();
        let origin = Position::new(7, ROOK_LONG_CASTLING_INITIAL_COLUMN);

        assert!(rook.is_valid_castling(origin));
    }

    #[test]
    fn test_is_valid_castling_non_initial_position() {
        let rook = Rook::new();
        let origin = Position::new(7, 3);

        assert!(!rook.is_valid_castling(origin));
    }

    #[test]
    fn test_is_valid_castling_denied_short() {
        let mut rook = Rook::new();
        rook.short_castling_available = false;
        let origin = Position::new(7, ROOK_SHORT_CASTLING_INITIAL_COLUMN);

        assert!(!rook.is_valid_castling(origin));
    }

    #[test]
    fn test_is_valid_castling_denied_long() {
        let mut rook = Rook::new();
        rook.long_castling_available = false;
        let origin = Position::new(7, ROOK_LONG_CASTLING_INITIAL_COLUMN);

        assert!(!rook.is_valid_castling(origin));
    }

    #[test]
    fn test_can_castle_success() {
        let board = setup_board(Some("tests/rook/only_rook_castling.txt"));
        let rook = Rook::new();
        let source = Position::new(7, ROOK_SHORT_CASTLING_INITIAL_COLUMN);
        let destination = Position::new(7, ROOK_SHORT_CASTLING_COLUMN);

        assert!(rook.can_castle(&board, source, destination));
    }

    #[test]
    fn test_can_castle_blocked_path() {
        let mut board = setup_board(Some("tests/rook/only_rook_castling.txt"));
        let rook = Rook::new();
        let source = Position::new(7, ROOK_SHORT_CASTLING_INITIAL_COLUMN);
        let destination = Position::new(7, ROOK_SHORT_CASTLING_COLUMN);

        board.add_piece(
            Piece::new(PieceType::Pawn(Pawn::new()), Color::White),
            Position::new(7, 5),
        );

        assert!(!rook.can_castle(&board, source, destination));
    }

    #[test]
    fn test_can_castle_denied_rights() {
        let board = setup_board(Some("tests/rook/only_rook_castling.txt"));
        let mut rook = Rook::new();
        rook.short_castling_available = false;
        let source = Position::new(7, ROOK_SHORT_CASTLING_INITIAL_COLUMN);
        let destination = Position::new(7, ROOK_SHORT_CASTLING_COLUMN);

        assert!(!rook.can_castle(&board, source, destination));
    }

    #[test]
    fn test_get_castle_move_white_short() {
        let (source, destination) = Rook::get_castle_move(Color::White, true);

        assert_eq!(source, Position::new(7, 7));
        assert_eq!(destination, Position::new(7, 5));
    }

    #[test]
    fn test_get_castle_move_white_long() {
        let (source, destination) = Rook::get_castle_move(Color::White, false);

        assert_eq!(source, Position::new(7, 0));
        assert_eq!(destination, Position::new(7, 3));
    }

    #[test]
    fn test_get_castle_move_black_short() {
        let (source, destination) = Rook::get_castle_move(Color::Black, true);

        assert_eq!(source, Position::new(0, 7));
        assert_eq!(destination, Position::new(0, 5));
    }

    #[test]
    fn test_get_castle_move_black_long() {
        let (source, destination) = Rook::get_castle_move(Color::Black, false);

        assert_eq!(source, Position::new(0, 0));
        assert_eq!(destination, Position::new(0, 3));
    }

    #[test]
    fn test_deny_castling_rights() {
        let mut rook = Rook::new();

        assert!(rook.short_castling_available);
        assert!(rook.long_castling_available);

        rook.deny_castling_rights();

        assert!(!rook.short_castling_available);
        assert!(!rook.long_castling_available);
    }

    #[test]
    fn test_rook_new_has_both_castling_rights() {
        let rook = Rook::new();

        assert!(rook.short_castling_available);
        assert!(rook.long_castling_available);
    }

    #[test]
    fn test_rook_default_has_both_castling_rights() {
        let rook = Rook::default();

        assert!(rook.short_castling_available);
        assert!(rook.long_castling_available);
    }
}
