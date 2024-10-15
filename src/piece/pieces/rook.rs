use crate::utils::types::Board;
use crate::utils::Position;
use std::cmp::max;

pub const SYMBOLS: [char; 2] = ['\u{2656}', '\u{265C}'];

pub fn can_move(board: &Board, origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    // Logical XNOR
    if (src_line == dest_line) == (src_col == dest_col) {
        return false;
    }

    let (horizontal_direction, vertical_direction) =
        get_directions(src_line, src_col, dest_line, dest_col);

    let nr_of_squares = max((dest_col - src_col).abs(), (dest_line - src_line).abs());

    check_clear_path(
        board,
        src_line,
        src_col,
        horizontal_direction,
        vertical_direction,
        nr_of_squares,
    )
}

// TODO reuse the can_move function above
pub fn attacks(board: &Board, origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    // Logical XNOR
    if (src_line == dest_line) == (src_col == dest_col) {
        return false;
    }

    let (horizontal_direction, vertical_direction) =
        get_directions(src_line, src_col, dest_line, dest_col);

    let nr_of_squares = max((dest_col - src_col).abs(), (dest_line - src_line).abs()) - 1;

    check_clear_path(
        board,
        src_line,
        src_col,
        horizontal_direction,
        vertical_direction,
        nr_of_squares,
    )
}

fn get_directions(src_line: i8, src_col: i8, dest_line: i8, dest_col: i8) -> (i8, i8) {
    // Avoid division by zero
    let horizontal_direction = if (dest_col - src_col) != 0 {
        (dest_col - src_col) / (dest_col - src_col).abs()
    }
    else {
        0
    };

    let vertical_direction = if (dest_line - src_line) != 0 {
        (dest_line - src_line) / (dest_line - src_line).abs()
    }
    else {
        0
    };
    
    (horizontal_direction, vertical_direction)
}

fn check_clear_path(
    board: &Board,
    src_line: i8,
    src_col: i8,
    horizontal_direction: i8,
    vertical_direction: i8,
    nr_of_squares: i8,
) -> bool {
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