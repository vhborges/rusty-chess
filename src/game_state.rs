use crate::Board;
use crate::errors::MoveError;
use crate::errors::constants::{INTERNAL_ERROR_01, INTERNAL_ERROR_02};
use crate::io::file_manager::initial_positions;
use crate::movement::{ChessPosition, Move, Position};
use crate::pgn::pgn_parser::parse_move;
use crate::pieces::types::{King, Rook};
use crate::pieces::{Color, Piece, PieceType};
use crate::utils::helper_functions::get_next_char;
use std::mem::{discriminant, swap};

#[derive(Clone, PartialEq, Debug)]
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
        if discriminant(&piece.piece_type) != discriminant(&piece_type) {
            return Ok(false);
        }
        if piece.color != self.turn {
            return Ok(false);
        }

        if capture {
            piece.attacks(&self.board, origin, destination, true, true)
        }
        else {
            piece.can_move(&self.board, origin, destination)
        }
    }

    /// Find and return the King and Rook moves (in that order) needed for castling
    pub fn find_castling_move(&self, is_short_castle: bool) -> Result<Move, MoveError> {
        let (king_source, king_destination) = King::get_castle_move(self.turn, is_short_castle);
        let king = self
            .board
            .get_piece(king_source)
            .ok_or(MoveError::InvalidCastle(
                "The King is no longer on its original square",
            ))?;

        let (rook_source, rook_destination) = Rook::get_castle_move(self.turn, is_short_castle);
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

        self.board.update_piece_state(next_move.source());
        if let Some(additional_move) = next_move.additional {
            self.board.update_piece_state(additional_move.source);
        }

        self.update_captured_pieces_list(next_move.destination());

        self.board.perform_move(next_move);

        self.turn.flip();

        Ok(())
    }

    fn verify_king_in_check(&mut self, next_move: Move) -> Result<(), MoveError> {
        let captured_piece = self.board.get_piece(next_move.destination());

        self.board.perform_move(next_move);

        let king_pos = self.get_king_pos(next_move.destination());
        let king_in_check = self.is_king_in_check(king_pos, self.turn);

        self.board.undo_move(next_move, captured_piece);

        if king_in_check {
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
        if let PieceType::King(_) = source_piece.piece_type {
            match self.turn {
                Color::White => self.white_king_position = next_move.destination(),
                Color::Black => self.black_king_position = next_move.destination(),
            }
        }
    }

    fn get_king_pos(&self, destination: Position) -> Position {
        match self.board.get_piece(destination).unwrap().piece_type {
            PieceType::King(_) => destination,
            _ => match self.turn {
                Color::White => self.white_king_position,
                Color::Black => self.black_king_position,
            },
        }
    }

    fn is_king_in_check(&self, king_pos: Position, color: Color) -> bool {
        for (piece, pos) in self
            .board
            .into_iter()
            .filter(|(piece, _)| piece.color != color)
        {
            if piece
                .attacks(&self.board, pos, king_pos, false, false)
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
                .unwrap_or_else(|_| panic!("Could not parse color character from line {line}"));

            let piece_type: PieceType = get_next_char(&line, &mut chars)
                .try_into()
                .unwrap_or_else(|_| panic!("Could not parse piece character from line {line}"));

            let chess_col = get_next_char(&line, &mut chars);

            let chess_line = get_next_char(&line, &mut chars);

            let piece_position = ChessPosition::new(chess_line, chess_col)
                .try_into()
                .unwrap_or_else(|_| {
                    panic!("Could not convert ChessPosition {chess_col}{chess_line} to Position")
                });

            if let PieceType::King(_) = piece_type {
                match piece_color {
                    Color::White => self.white_king_position = piece_position,
                    Color::Black => self.black_king_position = piece_position,
                }
            }

            if self.board.is_position_occupied(piece_position) {
                panic!(
                    "Duplicate piece position in initial setup: {chess_col}{chess_line} (line: {line})"
                );
            }

            self.board
                .add_piece(Piece::new(piece_type, piece_color), piece_position);
        }
        self.initialized = true;
    }

    fn validate_castling_path(&mut self, mut next_move: Move) -> Result<(), MoveError> {
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

    pub fn verify_checkmate(&mut self) -> bool {
        let (color, king_pos) = match self.turn {
            Color::White => (Color::White, self.white_king_position),
            Color::Black => (Color::Black, self.black_king_position),
        };

        if !self.is_king_in_check(king_pos, color) {
            return false;
        }

        // check if the King has a valid move to get out of check
        let king = self.board.get_piece(king_pos).unwrap();
        for dest in king.get_possible_moves(&self.board, king_pos) {
            let next_move = Move::new(king_pos, dest);
            let captured_piece = self.board.get_piece(next_move.destination());

            self.board.perform_move(next_move);
            let king_in_check = self.is_king_in_check(dest, color);
            self.board.undo_move(next_move, captured_piece);

            if !king_in_check {
                return false;
            }
        }

        // check if any other piece can cover the check
        let pieces_to_check: Vec<(Piece, Position)> = self
            .board
            .into_iter()
            .filter(|(piece, _)| {
                piece.color == self.turn && !matches!(piece.piece_type, PieceType::King(_))
            })
            .collect();

        for (piece, source) in pieces_to_check {
            // get the piece possible movements
            let possible_movements = piece.get_possible_moves(&self.board, source);

            // check if any of them can cover the check
            for dest in possible_movements {
                let next_move = Move::new(source, dest);
                let captured_piece = self.board.get_piece(next_move.destination());

                self.board.perform_move(next_move);
                let king_in_check = self.is_king_in_check(king_pos, color);
                self.board.undo_move(next_move, captured_piece);

                if !king_in_check {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_stalemate(&self) -> bool {
        for (piece, pos) in self
            .board
            .into_iter()
            .filter(|(piece_, _)| piece_.color == self.turn)
        {
            if !piece.get_possible_moves(self.board(), pos).is_empty() {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::setup_game_state;

    #[test]
    fn test_validate_castling_path_short_castle_success() {
        let mut game_state = setup_game_state(Some("tests/validate_castling_path_success.txt"));

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
        let mut game_state = setup_game_state(Some("tests/validate_castling_path_fail.txt"));

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
        let mut game_state = setup_game_state(Some("tests/validate_castling_path_success.txt"));

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
        let mut game_state = setup_game_state(Some("tests/validate_castling_path_fail.txt"));

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

    #[test]
    fn test_stalemate() {
        let mut game_state = setup_game_state(Some("tests/validate_stalemate.txt"));

        assert!(game_state.is_stalemate());
        assert!(!game_state.verify_checkmate());
    }

    #[test]
    fn test_not_stalemate_when_other_legal_move_exists() {
        let game_state = setup_game_state(Some("tests/validate_not_stalemate_with_legal_move.txt"));

        assert!(!game_state.is_stalemate());
    }

    #[test]
    fn test_undo_move_integrity() {
        let mut game_state = setup_game_state(None);

        let m = Move::new(Position::new(6, 4), Position::new(4, 4)); // e2 to e4
        let captured = game_state.board().get_piece(m.destination());

        let initial_board_snapshot = *game_state.board();
        game_state.board.perform_move(m);
        game_state.board.undo_move(m, captured);

        assert_eq!(game_state.board(), &initial_board_snapshot);
    }

    #[test]
    fn test_checkmate_scholars_mate() {
        let mut game_state = setup_game_state(Some("tests/scholars_mate.txt"));

        game_state.turn.flip();

        assert!(game_state.verify_checkmate());
    }

    #[test]
    fn test_checkmate_blocked_by_piece() {
        let mut game_state = setup_game_state(Some("tests/check_can_be_blocked.txt"));

        game_state.turn.flip();

        assert!(!game_state.verify_checkmate());
    }

    #[test]
    fn test_checkmate_king_escapes() {
        let mut game_state = setup_game_state(Some("tests/king_can_escape.txt"));

        game_state.turn.flip();

        assert!(!game_state.verify_checkmate());
    }

    #[test]
    #[should_panic(expected = "Duplicate piece position in initial setup")]
    fn test_initialize_panics_on_duplicate_square() {
        let _ = setup_game_state(Some("tests/duplicate_square.txt"));
    }

    #[test]
    fn test_capture_updates_captured_pieces_and_board_state() -> Result<(), MoveError> {
        let mut game_state = setup_game_state(None);
        game_state.handle_move("e4")?;
        game_state.handle_move("d5")?;

        let source = Position::new(4, 4);
        let destination = Position::new(3, 3);
        let moving_piece = game_state.get_piece(source).unwrap();

        game_state.handle_move("exd5")?;

        let destination_piece = game_state.get_piece(destination).unwrap();

        assert_eq!(
            discriminant(&moving_piece.piece_type),
            discriminant(&destination_piece.piece_type)
        );
        assert_eq!(moving_piece.color, destination_piece.color);
        assert!(game_state.get_piece(source).is_none());
        assert_eq!(game_state.captured_black_pieces().len(), 1);
        assert_eq!(game_state.captured_white_pieces().len(), 0);

        Ok(())
    }
}
