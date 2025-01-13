use crate::errors::MoveError;
use crate::io::file_manager::initial_positions;
use crate::pgn::pgn_parser::parse_move;
use crate::piece::pieces::{king, rook};
use crate::piece::{Piece, PieceType};
use crate::utils::constants::*;
use crate::utils::{Board, ChessPosition, Color, Move, Position};
use std::mem::swap;
use std::str::Chars;

pub struct GameState {
    board: Board,
    captured_white_pieces: Vec<Piece>,
    captured_black_pieces: Vec<Piece>,
    turn: Color,
    white_king_position: Position,
    black_king_position: Position,
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
        }
    }

    pub fn add_piece(&mut self, piece: Piece, pos: Position) {
        self.board[pos.line][pos.col] = Some(piece);
    }

    pub fn is_white_turn(&self) -> bool {
        self.turn == Color::White
    }

    pub fn is_black_turn(&self) -> bool {
        self.turn == Color::Black
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        self.board[position.line][position.col]
    }

    pub fn print_game(&self) {
        clearscreen::clear().expect("Failed to clear screen");

        for (line, line_chess) in self.board.iter().zip(LINES.iter()) {
            print!("{} ", line_chess);
            for maybe_piece in line {
                match maybe_piece {
                    Some(piece) => print!("{} ", piece),
                    None => print!("{} ", BLANK_SQUARE),
                }
            }
            println!();
        }

        print!("  ");

        for col_chess in COLUMNS {
            print!("{} ", col_chess);
        }

        println!();

        for piece in &self.captured_white_pieces {
            print!("{} ", piece)
        }

        println!();

        for piece in &self.captured_black_pieces {
            println!("{} ", piece)
        }
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
        for (line_index, line) in self.board.iter().enumerate() {
            for (col_index, maybe_piece) in line.iter().enumerate() {
                if let Some(piece) = maybe_piece {
                    let origin = Position::new(line_index, col_index);
                    if self.piece_matches(piece, piece_type, origin, destination, capture)? {
                        matching_positions.push(origin);
                    }
                }
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
        let king = self.get_piece(king_source).ok_or(MoveError::InvalidCastle(
            "The King is no longer on its original square",
        ))?;

        let (rook_source, rook_destination) = rook::get_castle_move(self.turn, is_short_castle);
        let rook = self.get_piece(rook_source).ok_or(MoveError::InvalidCastle(
            "The Rook is no longer on its original square",
        ))?;

        if !(king.can_castle(&self.board, king_source, king_destination)?
            && rook.can_castle(&self.board, rook_source, rook_destination)?)
        {
            return Err(MoveError::InvalidCastle("This move is not allowed"));
        }

        Ok(Move::new_with_castling(
            king_source,
            king_destination,
            rook_source,
            rook_destination,
        ))
    }

    pub fn handle_move(&mut self, str_move: &str) -> Result<(), MoveError> {
        let next_move = parse_move(self, str_move)?;

        let source_line = next_move.source().line;
        let source_col = next_move.source().col;

        let dest_line = next_move.destination().line;
        let dest_col = next_move.destination().col;

        if next_move.is_castling() {
            self.validate_castling_path(next_move)?;
        }

        self.verify_king_in_check(&next_move)?;

        self.update_king_position(&next_move);

        self.update_castling_rights(source_line, source_col);

        self.update_captured_pieces_list(dest_line, dest_col);

        Self::perform_move(&next_move, &mut self.board);

        self.turn.flip();

        Ok(())
    }

    fn verify_king_in_check(&self, next_move: &Move) -> Result<(), MoveError> {
        let mut temporary_board = self.board;
        Self::perform_move(next_move, &mut temporary_board);

        let (dest_line, dest_col) = (next_move.destination().line, next_move.destination().col);
        let king_pos = self.get_king_pos(dest_line, dest_col, temporary_board);

        if self.is_king_in_check(&temporary_board, king_pos) {
            return Err(MoveError::KingWouldBeInCheck);
        }

        Ok(())
    }

    fn update_captured_pieces_list(&mut self, dest_line: usize, dest_col: usize) {
        let dest_piece = self.board[dest_line][dest_col];
        if let Some(captured_piece) = dest_piece {
            match captured_piece.color {
                Color::White => self.captured_white_pieces.push(captured_piece),
                Color::Black => self.captured_black_pieces.push(captured_piece),
            }
        }
    }

    fn update_king_position(&mut self, next_move: &Move) {
        let (source_line, source_col) = (next_move.source().line, next_move.source().col);
        let source_piece = self.board[source_line][source_col].unwrap();
        if source_piece.piece_type == PieceType::King {
            match self.turn {
                Color::White => self.white_king_position = next_move.destination(),
                Color::Black => self.black_king_position = next_move.destination(),
            }
        }
    }

    fn get_king_pos(
        &self,
        dest_line: usize,
        dest_col: usize,
        temporary_board: [[Option<Piece>; 8]; 8],
    ) -> Position {
        if temporary_board[dest_line][dest_col].unwrap().piece_type == PieceType::King {
            Position::new(dest_line, dest_col)
        }
        else {
            match self.turn {
                Color::White => self.white_king_position,
                Color::Black => self.black_king_position,
            }
        }
    }

    fn perform_move(_move: &Move, temporary_board: &mut Board) {
        if _move.source() != _move.destination() {
            temporary_board[_move.destination().line][_move.destination().col] =
                temporary_board[_move.source().line][_move.source().col];
            temporary_board[_move.source().line][_move.source().col] = None;
        }

        if let Some(additional) = _move.additional {
            let source = additional.source;
            let dest = additional.destination;
            if source != dest {
                temporary_board[dest.line][dest.col] = temporary_board[source.line][source.col];
                temporary_board[source.line][source.col] = None;
            }
        }
    }

    fn is_king_in_check(&self, board: &Board, king_pos: Position) -> bool {
        for (line_index, line) in board.iter().enumerate() {
            for (col_index, piece) in line
                .iter()
                .enumerate()
                .filter(|(_, piece)| piece.is_some() && piece.unwrap().color != self.turn)
            {
                let piece_pos = Position::new(line_index, col_index);

                if piece
                    .unwrap()
                    .attacks(board, piece_pos, king_pos, false)
                    .expect(INTERNAL_ERROR_02)
                {
                    return true;
                }
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

            self.add_piece(Piece::new(piece_type, piece_color), piece_position);
        }
    }

    fn update_castling_rights(&mut self, source_line: usize, source_col: usize) {
        let piece = self.board[source_line][source_col].as_mut().unwrap();
        if piece.piece_type == PieceType::King {
            piece.long_castling_available = false;
            piece.short_castling_available = false;
        }
        else if piece.piece_type == PieceType::Rook {
            if source_col == ROOK_LONG_CASTLING_INITIAL_COLUMN {
                piece.long_castling_available = false;
            }
            else if source_col == ROOK_SHORT_CASTLING_INITIAL_COLUMN {
                piece.short_castling_available = false;
            }
        }
    }

    fn validate_castling_path(&self, mut next_move: Move) -> Result<(), MoveError> {
        next_move.additional = None;

        let (mut start, mut end) = (next_move.source().col, next_move.destination().col);
        if start > end {
            swap(&mut start, &mut end);
        }

        for col in start..=end {
            next_move.primary.destination.col = col;
            self.verify_king_in_check(&next_move)?
        }

        Ok(())
    }
}

fn get_next_char(line: &String, chars: &mut Chars) -> char {
    chars
        .next()
        .unwrap_or_else(|| panic!("Line {} is incomplete", line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::setup;

    #[test]
    fn test_update_castling_rights_king_move() {
        let mut game_state = setup(None);

        game_state.update_castling_rights(0, 4);

        assert!(!game_state.board[0][4].unwrap().short_castling_available);
        assert!(!game_state.board[0][4].unwrap().long_castling_available);
    }

    #[test]
    fn test_update_castling_rights_long_rook_move() {
        let mut game_state = setup(None);

        game_state.update_castling_rights(0, 0);

        assert!(game_state.board[0][0].unwrap().short_castling_available);
        assert!(!game_state.board[0][0].unwrap().long_castling_available);
    }

    #[test]
    fn test_update_castling_rights_short_rook_move() {
        let mut game_state = setup(None);

        game_state.update_castling_rights(0, 7);

        assert!(!game_state.board[0][7].unwrap().short_castling_available);
        assert!(game_state.board[0][7].unwrap().long_castling_available);
    }

    #[test]
    fn test_validate_castling_path_short_castle_success() {
        let game_state = setup(Some("tests/validate_castling_path_success.txt"));

        let king_source = Position::new(7, 4);
        let king_destination = Position::new(7, 6);
        let rook_source = Position::new(7, 7);
        let rook_destination = Position::new(7, 5);
        let next_move =
            Move::new_with_castling(king_source, king_destination, rook_source, rook_destination);
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
            Move::new_with_castling(king_source, king_destination, rook_source, rook_destination);
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
            Move::new_with_castling(king_source, king_destination, rook_source, rook_destination);
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
            Move::new_with_castling(king_source, king_destination, rook_source, rook_destination);
        let result = game_state.validate_castling_path(next_move);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::KingWouldBeInCheck);
    }
}
