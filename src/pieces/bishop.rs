use crate::utils::Position;
use crate::utils::types::Board;

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

// pub fn get_possible_moves(board: &Board, origin: Position) -> PossibleMoves {
//     let mut up = 0..origin.line;
//     let mut down = (origin.line + 1)..BOARD_SIZE;
//     let mut left = 0..origin.col;
//     let mut right = (origin.col + 1)..BOARD_SIZE;
// 
//     let mut possible_moves: PossibleMoves = Default::default();
// 
//     check_diagonal(board, &mut up, &mut right, &mut possible_moves);
// 
//     check_diagonal(board, &mut up, &mut left, &mut possible_moves);
// 
//     check_diagonal(board, &mut down, &mut right, &mut possible_moves);
// 
//     check_diagonal(board, &mut down, &mut left, &mut possible_moves);
// 
//     possible_moves
// }
// 
// fn check_diagonal(board: &Board, vertical_range: &mut Range<usize>, horizontal_range: &mut Range<usize>, possible_moves: &mut PossibleMoves) {
//     for (i, j) in vertical_range.by_ref().zip(horizontal_range.by_ref()) {
//         if board[i][j].is_some() {
//             break;
//         }
//         possible_moves[i][j] = true;
//     }
// }

pub fn can_move(board: &Board, origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    if (src_line == dest_line) || (src_col == dest_col) {
        return false;
    }

    if (src_line - dest_line).abs() != (src_col - dest_col).abs() {
        return false;
    }

    let horizontal_direction = (dest_col - src_col) / (dest_col - src_col).abs();
    let vertical_direction = (dest_line - src_line) / (dest_line - src_line).abs();

    let mut i = (src_line + vertical_direction) as usize;
    let mut j = (src_col + horizontal_direction) as usize;
    let nr_of_squares = (dest_col - src_col).abs() - 1;
    for _ in 0..nr_of_squares {
        if board[i][j].is_some() {
            return false;
        }

        i = (i as i8 + vertical_direction) as usize;
        j = (j as i8 + horizontal_direction) as usize;
    }

    return true;
}

pub fn attacks(board: &Board, origin: Position, destination: Position) -> bool {
    can_move(board, origin, destination)
}
