use std::cmp::max;
use crate::utils::Position;
use crate::utils::types::Board;

pub const SYMBOLS: [char; 2] = ['\u{2656}', '\u{265C}'];

pub fn can_move(board: &Board, origin: Position,  destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    // Logical XNOR
    if (src_line == dest_line) == (src_col == dest_col) {
        return false;
    }
    
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

    let mut i = (src_line + vertical_direction) as usize;
    let mut j = (src_col + horizontal_direction) as usize;
    let nr_of_squares = max((dest_col - src_col).abs(), (dest_line - src_line).abs());
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
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    // Logical XNOR
    if (src_line == dest_line) == (src_col == dest_col) {
        return false;
    }

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

    let mut i = (src_line + vertical_direction) as usize;
    let mut j = (src_col + horizontal_direction) as usize;
    let nr_of_squares = max((dest_col - src_col).abs(), (dest_line - src_line).abs()) - 1;
    for _ in 0..nr_of_squares {
        if board[i][j].is_some() {
            return false;
        }

        i = (i as i8 + vertical_direction) as usize;
        j = (j as i8 + horizontal_direction) as usize;
    }

    return true;
}
