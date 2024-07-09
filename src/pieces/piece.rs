use std::fmt::Display;

use super::{bishop, king, knight, pawn, queen, rook};
use crate::errors::{MoveError, PgnError};
use crate::utils::types::Board;
use crate::utils::{Color, Position};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

impl TryFrom<char> for PieceType {
    type Error = PgnError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' => Ok(PieceType::Bishop),
            'K' => Ok(PieceType::King),
            'N' => Ok(PieceType::Knight),
            'a'..='h' | 'P' => Ok(PieceType::Pawn),
            'Q' => Ok(PieceType::Queen),
            'R' => Ok(PieceType::Rook),
            _ => Err(PgnError::InvalidPiece(value)),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    symbol: char,
    pub piece_type: PieceType,
    pub color: Color,
    // possible_moves: PossibleMoves,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            symbol: Self::get_symbol(&piece_type, &color),
            piece_type,
            color,
            // possible_moves: Default::default(),
        }
    }

    // pub fn update_possible_moves(
    //     &mut self,
    //     board: &Board,
    //     origin: Position,
    // ) {
    //     match self.piece_type {
    //         PieceType::Bishop => self.possible_moves = bishop::get_possible_moves(board, origin),
    //         PieceType::King => self.possible_moves = king::get_possible_moves(board, origin),
    //         PieceType::Knight => self.possible_moves = knight::get_possible_moves(board, origin),
    //         PieceType::Pawn => self.possible_moves = pawn::get_possible_moves(board, origin),
    //         PieceType::Queen => self.possible_moves = queen::get_possible_moves(board, origin),
    //         PieceType::Rook => self.possible_moves = rook::get_possible_moves(board, origin),
    //     }
    // }
    //
    pub fn can_move(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
        capture: bool,
    ) -> Result<bool, MoveError> {
        self.validate_capture(&board[destination.line][destination.col], capture)?;

        match self.piece_type {
            PieceType::Bishop => Ok(bishop::can_move(board, origin, destination)),
            PieceType::King => Ok(king::can_move(origin, destination)),
            PieceType::Knight => Ok(knight::can_move(origin, destination)),
            PieceType::Pawn => Ok(pawn::can_move(board, self, origin, destination, capture)),
            PieceType::Queen => Ok(queen::can_move(board, origin, destination)),
            PieceType::Rook => Ok(rook::can_move(board, origin, destination)),
        }
    }

    // TODO rever o uso do método abaixo, se ele for usado sempre em caso de captura, então podemos eliminar o booleano "capture" do método acima e implementar lógicas de captura nas funções "attacks" de cada peça
    pub fn attacks(
        &self,
        board: &Board,
        origin: Position,
        destination: Position,
    ) -> bool {
        match self.piece_type {
            PieceType::Bishop => bishop::attacks(board, origin, destination),
            PieceType::King => king::attacks(origin, destination),
            PieceType::Knight => knight::attacks(origin, destination),
            PieceType::Pawn => pawn::attacks(self.color, origin, destination),
            PieceType::Queen => queen::attacks(board, origin, destination),
            PieceType::Rook => rook::attacks(board, origin, destination),
        }
    }

    fn validate_capture(&self, dest_piece: &Option<Piece>, capture: bool) -> Result<(), MoveError> {
        if capture {
            if dest_piece.is_none() {
                return Err(MoveError::InvalidCapture("Destination square is empty"));
            }
            if dest_piece.unwrap().color == self.color {
                return Err(MoveError::InvalidCapture(
                    "Cannot capture a piece of the same color",
                ));
            }
        }
        else if dest_piece.is_some() && dest_piece.unwrap().color != self.color {
            return Err(PgnError::MissingCaptureCharacter.into());
        }
        else if dest_piece.is_some() && dest_piece.unwrap().color == self.color {
            return Err(MoveError::SquareOccupied);
        }
        Ok(())
    }

    fn get_symbol(piece_type: &PieceType, color: &Color) -> char {
        let symbols = match piece_type {
            PieceType::Bishop => bishop::SYMBOLS,
            PieceType::King => king::SYMBOLS,
            PieceType::Knight => knight::SYMBOLS,
            PieceType::Pawn => pawn::SYMBOLS,
            PieceType::Queen => queen::SYMBOLS,
            PieceType::Rook => rook::SYMBOLS,
        };

        Self::get_symbol_for_color(color, symbols)
    }

    fn get_symbol_for_color(color: &Color, symbols: [char; 2]) -> char {
        match color {
            Color::White => symbols[0],
            Color::Black => symbols[1],
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
