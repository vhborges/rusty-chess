use crate::pieces::Piece;
use crate::utils::constants::board;
use crate::utils::Position;

pub type Board = [[Option<Box<dyn Piece>>; board::SIZE]; board::SIZE];

pub struct GameState {
    board: Board,
    captured_white_pieces: Vec<Box<dyn Piece>>,
    captured_black_pieces: Vec<Box<dyn Piece>>,
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

    pub fn captured_white_pieces(&self) -> &Vec<Box<dyn Piece>> {
        &self.captured_white_pieces
    }

    pub fn captured_black_pieces(&self) -> &Vec<Box<dyn Piece>> {
        &self.captured_black_pieces
    }

    pub fn add_piece(&mut self, piece: Box<dyn Piece>, position: Position) {
        self.board[*position.line()][*position.col()] = Option::Some(piece);
    }
}
