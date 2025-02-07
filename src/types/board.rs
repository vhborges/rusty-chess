use crate::piece::Piece;
use crate::types::Position;
use crate::utils::constants::BOARD_SIZE;

type InternalBoard = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

#[derive(Copy, Clone)]
pub struct Board {
    board: InternalBoard,
    curr_pos: Position,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: Default::default(),
            curr_pos: Position::new(0, 0),
        }
    }
    
    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        self.board[position.line][position.col]
    }

    pub fn update_piece(&mut self, position: Position, piece: Piece) {
        self.board[position.line][position.col] = Some(piece);
    }
    
    pub fn add_piece(&mut self, piece: Piece, pos: Position) {
        self.board[pos.line][pos.col] = Some(piece);
    }
    
    pub fn is_position_occupied(&self, position: Position) -> bool {
        self.board[position.line][position.col].is_some()
    }
    
    pub fn move_piece(&mut self, origin: Position, destination: Position) {
        self.board[destination.line][destination.col] =
            self.get_piece(origin);
        
        self.board[origin.line][origin.col] = None;
    }
    
    pub fn is_path_clear(&self, path: Vec<Position>) -> bool {
        for pos in path {
            if self.board[pos.line][pos.col].is_some() {
                return false
            }
        }
        true
    }

    // pub fn iter(&self) -> Iter<'_, [Option<Piece>; BOARD_SIZE]> {
    //     self.board.iter()
    // }
}

impl Iterator for Board {
    type Item = (Piece, Position);

    fn next(&mut self) -> Option<Self::Item> {
        for pos in self.curr_pos {
            if let Some(piece) = self.get_piece(pos) {
                return Some((piece, pos))
            }
        }
        
        None
    }
}

struct PieceIterator {
    curr_pos: Position,
}

impl Iterator for PieceIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}