use crate::pieces::PieceType;
use crate::utils::types::Board;
use crate::utils::Position;

pub const SYMBOLS: [char; 2] = ['\u{2659}', '\u{265F}'];

pub fn can_move(origin: Position, destination: Position, board: Board) -> bool {
    let (line, col) = (*origin.line(), *origin.col());

    assert!(
        board[line][col].is_some() && board[line][col].unwrap().piece_type == PieceType::Pawn,
        "Internal error 01: Incorrect piece type or position"
    );

    let (dest_line, dest_col) = (*destination.line(), *destination.col());

    dest_col == col && (dest_line as i8 - line as i8).abs() == 1
}
