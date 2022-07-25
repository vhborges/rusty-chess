use crate::board::Board;
use crate::pieces::Piece;
use crate::utils::{Color, Position};

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
}
