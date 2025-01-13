use crate::utils::{Board, Position};

pub const SYMBOLS: [char; 2] = ['\u{2657}', '\u{265D}'];

pub fn can_move(board: &Board, origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    if !is_move_valid(src_line, src_col, dest_line, dest_col) {
        return false;
    }

    let nr_of_squares = (dest_col - src_col).abs();
    if !is_path_clear(board, src_line, src_col, dest_line, dest_col, nr_of_squares) {
        return false;
    }

    true
}

pub fn attacks(board: &Board, origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    if !is_move_valid(src_line, src_col, dest_line, dest_col) {
        return false;
    }

    let nr_of_squares = (dest_col - src_col).abs() - 1;
    if !is_path_clear(board, src_line, src_col, dest_line, dest_col, nr_of_squares) {
        return false;
    }

    true
}

fn is_move_valid(src_line: i8, src_col: i8, dest_line: i8, dest_col: i8) -> bool {
    if (src_line == dest_line) || (src_col == dest_col) {
        return false;
    }

    if (src_line - dest_line).abs() != (src_col - dest_col).abs() {
        return false;
    }

    true
}

fn is_path_clear(
    board: &Board,
    src_line: i8,
    src_col: i8,
    dest_line: i8,
    dest_col: i8,
    nr_of_squares: i8,
) -> bool {
    let horizontal_direction = (dest_col - src_col) / (dest_col - src_col).abs();
    let vertical_direction = (dest_line - src_line) / (dest_line - src_line).abs();

    let mut i = (src_line + vertical_direction) as usize;
    let mut j = (src_col + horizontal_direction) as usize;
    for _ in 0..nr_of_squares {
        if board[i][j].is_some() {
            return false;
        }

        i = (i as i8 + vertical_direction) as usize;
        j = (j as i8 + horizontal_direction) as usize;
    }

    true
}
