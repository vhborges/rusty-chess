use super::super::Color;
use super::super::Piece;
use crate::Board;
use crate::movement::{Position, PositionI8};

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pawn {
    pub allow_two_rows: bool,
}

impl Default for Pawn {
    fn default() -> Self {
        Self::new()
    }
}

impl Pawn {
    pub fn new() -> Self {
        Pawn {
            allow_two_rows: true,
        }
    }

    pub fn can_move(
        &self,
        piece: &Piece,
        board: &Board,
        source: Position,
        destination: Position,
    ) -> bool {
        let src: PositionI8 = source.into();
        let dest: PositionI8 = destination.into();

        let vertical_distance = match piece.color {
            Color::White => src.line - dest.line,
            Color::Black => dest.line - src.line,
        };

        if dest.col != src.col {
            return false;
        }

        (vertical_distance == 1 || (self.allow_two_rows && vertical_distance == 2))
            && board.is_path_clear(src, dest, vertical_distance as usize)
    }

    pub fn attacks(piece_color: Color, source: Position, destination: Position) -> bool {
        let src: PositionI8 = source.into();
        let dest: PositionI8 = destination.into();

        let vertical_distance = dest.line - src.line;
        let abs_horizontal_distance = (dest.col - src.col).abs();

        match piece_color {
            Color::White => vertical_distance == -1 && abs_horizontal_distance == 1,
            Color::Black => vertical_distance == 1 && abs_horizontal_distance == 1,
        }
    }

    pub fn get_possible_moves(
        &self,
        piece_color: Color,
        board: &Board,
        source: Position,
    ) -> Vec<Position> {
        let mut result = Vec::new();

        let direction = match piece_color {
            Color::White => -1,
            Color::Black => 1,
        };

        let mut dest_i8: PositionI8 = source.into();

        // 1 row
        dest_i8.line += direction;
        if let Ok(dest) = dest_i8.try_into()
            && !board.is_position_occupied(dest)
        {
            result.push(dest)
        }
        else {
            return result;
        }

        // 2 rows
        if self.allow_two_rows {
            dest_i8.line += direction;
            if let Ok(dest) = dest_i8.try_into()
                && !board.is_position_occupied(dest)
            {
                result.push(dest)
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::PieceType;
    use crate::pieces::types::Rook;
    use crate::utils::test_helper::setup_board;

    #[test]
    fn test_white_pawn_single_step() {
        let board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let pawn = Pawn::new();
        let piece = Piece::new(PieceType::Pawn(pawn), Color::White);
        let source = Position::new(6, 4);
        let destination = Position::new(5, 4);

        assert!(pawn.can_move(&piece, &board, source, destination));
    }

    #[test]
    fn test_white_pawn_double_step_from_start() {
        let board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let pawn = Pawn::new();
        let piece = Piece::new(PieceType::Pawn(pawn), Color::White);
        let source = Position::new(6, 4);
        let destination = Position::new(4, 4);

        assert!(pawn.can_move(&piece, &board, source, destination));
    }

    #[test]
    fn test_black_pawn_single_step() {
        let board = setup_board(Some("tests/pawn/only_black_pawn.txt"));
        let pawn = Pawn::new();
        let piece = Piece::new(PieceType::Pawn(pawn), Color::Black);
        let source = Position::new(1, 4);
        let destination = Position::new(2, 4);

        assert!(pawn.can_move(&piece, &board, source, destination));
    }

    #[test]
    fn test_black_pawn_double_step_from_start() {
        let board = setup_board(Some("tests/pawn/only_black_pawn.txt"));
        let pawn = Pawn::new();
        let piece = Piece::new(PieceType::Pawn(pawn), Color::Black);
        let source = Position::new(1, 4);
        let destination = Position::new(3, 4);

        assert!(pawn.can_move(&piece, &board, source, destination));
    }

    #[test]
    fn test_pawn_sideways_move_returns_false() {
        let board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let pawn = Pawn::new();
        let piece = Piece::new(PieceType::Pawn(pawn), Color::White);
        let source = Position::new(6, 4);
        let destination = Position::new(6, 5);

        assert!(!pawn.can_move(&piece, &board, source, destination));
    }

    #[test]
    fn test_pawn_backward_move_returns_false() {
        let board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let pawn = Pawn::new();
        let piece = Piece::new(PieceType::Pawn(pawn), Color::White);
        let source = Position::new(6, 4);
        let destination = Position::new(7, 4);

        assert!(!pawn.can_move(&piece, &board, source, destination));
    }

    #[test]
    fn test_pawn_blocked_by_piece_in_front() {
        let mut board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let pawn = Pawn::new();
        let piece = Piece::new(PieceType::Pawn(pawn), Color::White);
        let source = Position::new(6, 4);

        board.add_piece(
            Piece::new(PieceType::Rook(Rook::new()), Color::Black),
            Position::new(5, 4),
        );

        assert!(!pawn.can_move(&piece, &board, source, Position::new(5, 4)));
    }

    #[test]
    fn test_pawn_double_step_mid_path_blocked() {
        let mut board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let pawn = Pawn::new();
        let piece = Piece::new(PieceType::Pawn(pawn), Color::White);
        let source = Position::new(6, 4);

        board.add_piece(
            Piece::new(PieceType::Rook(Rook::new()), Color::Black),
            Position::new(5, 4),
        );

        assert!(!pawn.can_move(&piece, &board, source, Position::new(4, 4)));
    }

    #[test]
    fn test_pawn_allow_two_rows_false_blocks_double_step() {
        let board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let mut pawn = Pawn::new();
        pawn.allow_two_rows = false;
        let piece = Piece::new(PieceType::Pawn(pawn), Color::White);
        let source = Position::new(6, 4);

        assert!(pawn.can_move(&piece, &board, source, Position::new(5, 4)));
        assert!(!pawn.can_move(&piece, &board, source, Position::new(4, 4)));
    }

    #[test]
    fn test_white_pawn_attacks_diagonal() {
        assert!(Pawn::attacks(
            Color::White,
            Position::new(4, 4),
            Position::new(3, 3)
        ));
        assert!(Pawn::attacks(
            Color::White,
            Position::new(4, 4),
            Position::new(3, 5)
        ));
    }

    #[test]
    fn test_black_pawn_attacks_diagonal() {
        assert!(Pawn::attacks(
            Color::Black,
            Position::new(4, 4),
            Position::new(5, 3)
        ));
        assert!(Pawn::attacks(
            Color::Black,
            Position::new(4, 4),
            Position::new(5, 5)
        ));
    }

    #[test]
    fn test_pawn_attacks_non_diagonal_returns_false() {
        assert!(!Pawn::attacks(
            Color::White,
            Position::new(4, 4),
            Position::new(5, 4)
        ));
        assert!(!Pawn::attacks(
            Color::Black,
            Position::new(4, 4),
            Position::new(3, 4)
        ));
        assert!(!Pawn::attacks(
            Color::White,
            Position::new(4, 4),
            Position::new(4, 5)
        ));
    }

    #[test]
    fn test_get_possible_moves_unblocked_white_pawn() {
        let board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let pawn = Pawn::new();
        let source = Position::new(6, 4);

        let result = pawn.get_possible_moves(Color::White, &board, source);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&Position::new(5, 4)));
        assert!(result.contains(&Position::new(4, 4)));
    }

    #[test]
    fn test_get_possible_moves_blocked_white_pawn() {
        let mut board = setup_board(Some("tests/pawn/only_white_pawn.txt"));
        let pawn = Pawn::new();
        let source = Position::new(6, 4);

        board.add_piece(
            Piece::new(PieceType::Rook(Rook::new()), Color::Black),
            Position::new(5, 4),
        );

        let result = pawn.get_possible_moves(Color::White, &board, source);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_possible_moves_after_first_move() {
        let board = setup_board(Some("tests/pawn/only_white_pawn_moved.txt"));
        let mut pawn = Pawn::new();
        pawn.allow_two_rows = false;
        let source = Position::new(5, 4);

        let result = pawn.get_possible_moves(Color::White, &board, source);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&Position::new(4, 4)));
    }
}
