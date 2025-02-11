use crate::errors::MoveError;
use crate::errors::constants::{INTERNAL_ERROR_01, INTERNAL_ERROR_02};
use crate::io::file_manager::initial_positions;
use crate::pgn::pgn_parser::parse_move;
use crate::piece::pieces::rook::{
    ROOK_LONG_CASTLING_INITIAL_COLUMN, ROOK_SHORT_CASTLING_INITIAL_COLUMN,
};
use crate::piece::pieces::{king, rook};
use crate::piece::{Piece, PieceType};
use crate::types::{Board, ChessPosition, Color, Move, Position};
use crate::utils::helper_functions::{get_next_char, perform_move};
use std::mem::swap;

pub struct GameState {
    board: Board,
    captured_white_pieces: Vec<Piece>,
    captured_black_pieces: Vec<Piece>,
    turn: Color,
    white_king_position: Position,
    black_king_position: Position,
    initialized: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Default::default(),
            captured_white_pieces: Vec::new(),
            captured_black_pieces: Vec::new(),
            turn: Color::White,
            white_king_position: Default::default(),
            black_king_position: Default::default(),
            initialized: Default::default(),
        }
    }

    pub fn captured_white_pieces(&self) -> &Vec<Piece> {
        &self.captured_white_pieces
    }

    pub fn captured_black_pieces(&self) -> &Vec<Piece> {
        &self.captured_black_pieces
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn is_white_turn(&self) -> bool {
        self.turn == Color::White
    }

    pub fn is_black_turn(&self) -> bool {
        self.turn == Color::Black
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        self.board.get_piece(position)
    }

    /// Given the piece type, the destination square, the disambiguation character and whether
    /// it's a capture, find the piece that best matches these parameters and return its position
    pub fn find_piece_position(
        &self,
        piece_type: PieceType,
        destination: Position,
        disambiguation: Option<char>,
        capture: bool,
    ) -> Result<Position, MoveError> {
        let mut matching_positions = Vec::new();
        for (piece, src_pos) in &self.board {
            if self.piece_matches(&piece, piece_type, src_pos, destination, capture)? {
                matching_positions.push(src_pos);
            }
        }

        if matching_positions.is_empty() {
            return Err(MoveError::NoPieceAvailable);
        }
        if matching_positions.len() > 1 {
            let Some(disambiguation) = disambiguation
            else {
                return Err(MoveError::MoreThanOnePieceAvailable);
            };

            matching_positions.retain(|pos| -> bool {
                let chess_pos: ChessPosition = (*pos).try_into().expect(INTERNAL_ERROR_01);
                disambiguation == chess_pos.line || disambiguation == chess_pos.col
            });

            if matching_positions.len() != 1 {
                return Err(MoveError::MoreThanOnePieceAvailable);
            }
        }

        Ok(matching_positions[0])
    }

    fn piece_matches(
        &self,
        piece: &Piece,
        piece_type: PieceType,
        origin: Position,
        destination: Position,
        capture: bool,
    ) -> Result<bool, MoveError> {
        if piece.piece_type != piece_type {
            return Ok(false);
        }
        if piece.color != self.turn {
            return Ok(false);
        }

        if capture {
            piece.attacks(&self.board, origin, destination, true)
        }
        else {
            piece.can_move(&self.board, origin, destination)
        }
    }

    /// Find and return the King and Rook moves (in that order) needed for castling
    pub fn find_castling_move(&self, is_short_castle: bool) -> Result<Move, MoveError> {
        let (king_source, king_destination) = king::get_castle_move(self.turn, is_short_castle);
        let king = self
            .board
            .get_piece(king_source)
            .ok_or(MoveError::InvalidCastle(
                "The King is no longer on its original square",
            ))?;

        let (rook_source, rook_destination) = rook::get_castle_move(self.turn, is_short_castle);
        let rook = self
            .board
            .get_piece(rook_source)
            .ok_or(MoveError::InvalidCastle(
                "The Rook is no longer on its original square",
            ))?;

        if !(king.can_castle(&self.board, king_source, king_destination)?
            && rook.can_castle(&self.board, rook_source, rook_destination)?)
        {
            return Err(MoveError::InvalidCastle("This move is not allowed"));
        }

        Ok(Move::with_castling(
            king_source,
            king_destination,
            rook_source,
            rook_destination,
        ))
    }

    pub fn handle_move(&mut self, str_move: &str) -> Result<(), MoveError> {
        if !self.initialized {
            panic!("Should call 'initialize' before 'handle_move'");
        }

        let next_move = parse_move(self, str_move)?;

        if next_move.is_castling() {
            self.validate_castling_path(next_move)?;
        }

        self.verify_king_in_check(next_move)?;

        self.update_king_position(next_move);

        self.update_castling_rights(next_move.source());

        self.update_captured_pieces_list(next_move.destination());

        perform_move(next_move, &mut self.board);

        self.turn.flip();

        Ok(())
    }

    fn verify_king_in_check(&self, next_move: Move) -> Result<(), MoveError> {
        let mut temporary_board = self.board;
        perform_move(next_move, &mut temporary_board);

        let king_pos = self.get_king_pos(temporary_board, next_move.destination());

        if self.is_king_in_check(&temporary_board, king_pos) {
            return Err(MoveError::KingWouldBeInCheck);
        }

        Ok(())
    }

    fn update_captured_pieces_list(&mut self, pos: Position) {
        let piece = self.board.get_piece(pos);
        if let Some(captured_piece) = piece {
            match captured_piece.color {
                Color::White => self.captured_white_pieces.push(captured_piece),
                Color::Black => self.captured_black_pieces.push(captured_piece),
            }
        }
    }

    fn update_king_position(&mut self, next_move: Move) {
        let source_piece = self.board.get_piece(next_move.source()).unwrap();
        if source_piece.piece_type == PieceType::King {
            match self.turn {
                Color::White => self.white_king_position = next_move.destination(),
                Color::Black => self.black_king_position = next_move.destination(),
            }
        }
    }

    fn get_king_pos(&self, temporary_board: Board, destination: Position) -> Position {
        if temporary_board.get_piece(destination).unwrap().piece_type == PieceType::King {
            destination
        }
        else {
            match self.turn {
                Color::White => self.white_king_position,
                Color::Black => self.black_king_position,
            }
        }
    }

    fn is_king_in_check(&self, board: &Board, king_pos: Position) -> bool {
        for (piece, pos) in board {
            if piece.color != self.turn
                && piece
                    .attacks(board, pos, king_pos, false)
                    .expect(INTERNAL_ERROR_02)
            {
                return true;
            }
        }

        false
    }

    pub fn initialize(&mut self, positions_file: Option<&str>) {
        for wrapped_line in initial_positions(positions_file) {
            let line = wrapped_line.expect("Error reading file line");
            let mut chars = line.chars();

            let piece_color: Color = get_next_char(&line, &mut chars)
                .try_into()
                .unwrap_or_else(|_| panic!("Could not parse color character from line {}", line));

            let piece_type: PieceType = get_next_char(&line, &mut chars)
                .try_into()
                .unwrap_or_else(|_| panic!("Could not parse piece character from line {}", line));

            let chess_col = get_next_char(&line, &mut chars);

            let chess_line = get_next_char(&line, &mut chars);

            let piece_position = ChessPosition::new(chess_line, chess_col)
                .try_into()
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not convert ChessPosition {}{} to Position",
                        chess_col, chess_line
                    )
                });

            if piece_type == PieceType::King {
                match piece_color {
                    Color::White => self.white_king_position = piece_position,
                    Color::Black => self.black_king_position = piece_position,
                }
            }

            self.board
                .add_piece(Piece::new(piece_type, piece_color), piece_position);
        }
        self.initialized = true;
    }

    fn update_castling_rights(&mut self, pos: Position) {
        let mut piece = self.board.get_piece(pos).unwrap();

        if piece.piece_type == PieceType::King {
            piece.long_castling_available = false;
            piece.short_castling_available = false;
        }
        else if piece.piece_type == PieceType::Rook {
            if pos.col == ROOK_LONG_CASTLING_INITIAL_COLUMN {
                piece.long_castling_available = false;
            }
            else if pos.col == ROOK_SHORT_CASTLING_INITIAL_COLUMN {
                piece.short_castling_available = false;
            }
        }

        self.board.update_piece(pos, piece);
    }

    fn validate_castling_path(&self, mut next_move: Move) -> Result<(), MoveError> {
        next_move.additional = None;

        let (mut start, mut end) = (next_move.source().col, next_move.destination().col);
        if start > end {
            swap(&mut start, &mut end);
        }

        for col in start..=end {
            next_move.primary.destination.col = col;
            self.verify_king_in_check(next_move)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::setup;

    #[test]
    fn test_update_castling_rights_king_move() {
        let mut game_state = setup(None);

        game_state.update_castling_rights(Position::new(0, 4));

        let piece = game_state.board.get_piece(Position::new(0, 4)).unwrap();
        assert!(!piece.short_castling_available);
        assert!(!piece.long_castling_available);
    }

    #[test]
    fn test_update_castling_rights_long_rook_move() {
        let mut game_state = setup(None);

        game_state.update_castling_rights(Position::new(0, 0));

        let piece = game_state.board.get_piece(Position::new(0, 0)).unwrap();
        assert!(piece.short_castling_available);
        assert!(!piece.long_castling_available);
    }

    #[test]
    fn test_update_castling_rights_short_rook_move() {
        let mut game_state = setup(None);

        game_state.update_castling_rights(Position::new(0, 7));

        let piece = game_state.board.get_piece(Position::new(0, 7)).unwrap();
        assert!(!piece.short_castling_available);
        assert!(piece.long_castling_available);
    }

    #[test]
    fn test_validate_castling_path_short_castle_success() {
        let game_state = setup(Some("tests/validate_castling_path_success.txt"));

        let king_source = Position::new(7, 4);
        let king_destination = Position::new(7, 6);
        let rook_source = Position::new(7, 7);
        let rook_destination = Position::new(7, 5);
        let next_move =
            Move::with_castling(king_source, king_destination, rook_source, rook_destination);
        let result = game_state.validate_castling_path(next_move);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_castling_path_short_castle_fail() {
        let game_state = setup(Some("tests/validate_castling_path_fail.txt"));

        let king_source = Position::new(7, 4);
        let king_destination = Position::new(7, 6);
        let rook_source = Position::new(7, 7);
        let rook_destination = Position::new(7, 5);
        let next_move =
            Move::with_castling(king_source, king_destination, rook_source, rook_destination);
        let result = game_state.validate_castling_path(next_move);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::KingWouldBeInCheck);
    }

    #[test]
    fn test_validate_castling_path_long_castle_success() {
        let game_state = setup(Some("tests/validate_castling_path_success.txt"));

        let king_source = Position::new(7, 4);
        let king_destination = Position::new(7, 2);
        let rook_source = Position::new(7, 0);
        let rook_destination = Position::new(7, 3);
        let next_move =
            Move::with_castling(king_source, king_destination, rook_source, rook_destination);
        let result = game_state.validate_castling_path(next_move);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_castling_path_long_castle_fail() {
        let game_state = setup(Some("tests/validate_castling_path_fail.txt"));

        let king_source = Position::new(7, 4);
        let king_destination = Position::new(7, 2);
        let rook_source = Position::new(7, 0);
        let rook_destination = Position::new(7, 3);
        let next_move =
            Move::with_castling(king_source, king_destination, rook_source, rook_destination);
        let result = game_state.validate_castling_path(next_move);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::KingWouldBeInCheck);
    }
}
