use std::process::Command;

use crate::io::{get_next_char, initial_positions};
use crate::pieces::{Piece, PieceType};
use crate::utils::types::Board;
use crate::utils::{constants, ChessPosition, Color, Position};

pub struct GameState {
    board: Board,
    captured_white_pieces: Vec<Piece>,
    captured_black_pieces: Vec<Piece>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Default::default(),
            captured_white_pieces: Vec::new(),
            captured_black_pieces: Vec::new(),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn captured_white_pieces(&self) -> &Vec<Piece> {
        &self.captured_white_pieces
    }

    pub fn captured_black_pieces(&self) -> &Vec<Piece> {
        &self.captured_black_pieces
    }

    pub fn add_piece(&mut self, piece: Piece) {
        let line = *piece.position().line();
        let col = *piece.position().col();

        self.board[line][col] = Some(piece);
    }

    pub fn move_piece(&mut self, source: Position, dest: Position) {
        let source_line = *source.line();
        let source_col = *source.col();
        let source_piece = self.board[source_line][source_col];

        if source_piece.is_none() {
            return;
        }

        let dest_line = *dest.line();
        let dest_col = *dest.col();

        let dest_piece = self.board[dest_line][dest_col];

        if let Some(captured_piece) = dest_piece {
            match captured_piece.color() {
                Color::White => self.captured_white_pieces.push(captured_piece),
                Color::Black => self.captured_black_pieces.push(captured_piece),
            }
        }

        self.board[source_line][source_col] = None;
        self.board[dest_line][dest_col] = source_piece;
    }

    pub fn initialize(&mut self) {
        for wrapped_line in initial_positions() {
            let line = wrapped_line.expect("Error reading file line");
            let mut chars = line.chars();

            let piece_color: Color = get_next_char(&line, &mut chars)
                .try_into()
                .expect(format!("Could not parse color character from line {}", line).as_str());

            let piece_type: PieceType = get_next_char(&line, &mut chars)
                .try_into()
                .expect(format!("Could not parse piece character from line {}", line).as_str());

            let chess_col = get_next_char(&line, &mut chars);

            let chess_line = get_next_char(&line, &mut chars);

            let piece_position = ChessPosition::new(chess_line, chess_col).try_into().expect(
                format!(
                    "Could not convert ChessPosition {}{} to Position",
                    chess_col, chess_line
                )
                .as_str(),
            );

            self.add_piece(Piece::new(piece_type, piece_color, piece_position))
        }
    }

    pub fn print(&self) {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");

        for (line, line_chess) in self.board().iter().zip(constants::LINES.iter()) {
            print!("{} ", line_chess);
            for opt_piece in line {
                match opt_piece {
                    Some(piece) => print!("{} ", piece),
                    None => print!("{} ", constants::BLANK_SQUARE),
                }
            }
            println!();
        }

        print!("  ");

        for col_chess in constants::COLUMNS {
            print!("{} ", col_chess);
        }

        println!();

        for piece in self.captured_white_pieces() {
            print!("{} ", piece)
        }

        println!();

        for piece in self.captured_black_pieces() {
            print!("{} ", piece)
        }

        println!()
    }
}
