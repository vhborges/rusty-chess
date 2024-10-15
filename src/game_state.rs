use crate::errors::MoveError;
use crate::io::file_manager::initial_positions;
use crate::piece::{Piece, PieceType};
use crate::utils::constants::{BLANK_SQUARE, COLUMNS, INTERNAL_ERROR_01, INTERNAL_ERROR_02, LINES};
use crate::utils::types::Move;
use crate::utils::{pgn::pgn_utils::parse_move, types::Board, ChessPosition, Color, Position};
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
            for opt_piece in line {
                match opt_piece {
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

    pub fn find_piece_position(
        &self,
        piece_type: PieceType,
        destination: Position,
        opt_disambiguation: Option<char>,
        capture: bool,
        castling: bool,
    ) -> Result<Position, MoveError> {
        let mut matching_positions = Vec::new();
        for (line_index, line) in self.board.iter().enumerate() {
            for (col_index, opt_piece) in line.iter().enumerate() {
                if let Some(piece) = opt_piece {
                    let origin = Position::new(line_index, col_index);
                    if self.piece_matches(piece, piece_type, origin, destination, capture, castling)? {
                        matching_positions.push(origin);
                    }
                }
            }
        }

        if matching_positions.is_empty() {
            return Err(MoveError::NoPieceAvailable);
        }
        if matching_positions.len() > 1 {
            let Some(disambiguation) = opt_disambiguation
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
        castling: bool,
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
            piece.can_move(&self.board, origin, destination, castling)
        }
    }

    pub fn move_piece(&mut self, str_move: &str) -> Result<(), MoveError> {
        let next_move = parse_move(self, str_move)?;

        let source_line = next_move.source.line;
        let source_col = next_move.source.col;

        let dest_line = next_move.destination.line;
        let dest_col = next_move.destination.col;

        self.verify_king_in_check(&next_move, dest_line, dest_col)?;

        self.update_king_position(&next_move, source_line, source_col);

        self.update_captured_pieces_list(dest_line, dest_col);

        Self::perform_move(&next_move, &mut self.board);

        self.turn.flip();

        Ok(())
    }

    fn verify_king_in_check(
        &mut self,
        next_move: &Move,
        dest_line: usize,
        dest_col: usize,
    ) -> Result<(), MoveError> {
        let mut temporary_board = self.board;
        Self::perform_move(next_move, &mut temporary_board);

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

    fn update_king_position(&mut self, next_move: &Move, source_line: usize, source_col: usize) {
        let source_piece = self.board[source_line][source_col].unwrap();
        if source_piece.piece_type == PieceType::King {
            match self.turn {
                Color::White => self.white_king_position = next_move.destination,
                Color::Black => self.black_king_position = next_move.destination,
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
        temporary_board[_move.destination.line][_move.destination.col] =
            temporary_board[_move.source.line][_move.source.col];
        temporary_board[_move.source.line][_move.source.col] = None;
    }

    fn is_king_in_check(&self, board: &Board, king_pos: Position) -> bool {
        for (line_index, line) in board.iter().enumerate() {
            for (col_index, opt_piece) in line.iter().enumerate() {
                let Some(piece) = opt_piece
                else {
                    continue;
                };

                if piece.color == self.turn {
                    continue;
                }

                let piece_pos = Position::new(line_index, col_index);

                if piece
                    .attacks(board, piece_pos, king_pos, false)
                    .expect(INTERNAL_ERROR_02)
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn initialize(&mut self) {
        for wrapped_line in initial_positions() {
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
}

fn get_next_char(line: &String, chars: &mut Chars) -> char {
    chars
        .next()
        .unwrap_or_else(|| panic!("Line {} is incomplete", line))
}