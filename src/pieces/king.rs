use crate::utils::constants::BOARD_SIZE;
use crate::utils::Position;

pub const SYMBOLS: [char; 2] = ['\u{2654}', '\u{265A}'];

pub fn can_move(origin: Position, destination: Position) -> bool {
    let (src_line, src_col) = (origin.line as i8, origin.col as i8);
    let (dest_line, dest_col) = (destination.line as i8, destination.col as i8);

    for i in -1..2_i8 {
        for j in -1..2_i8 {
            if i == 0 && j == 0 {
                // Don't allow the King to stay where it is
                continue;
            }
            if (src_line + i) < 0
                || (src_line + i) >= BOARD_SIZE as i8
                || (src_col + j) < 0
                || (src_col + j) >= BOARD_SIZE as i8
            {
                continue;
            }
            if (src_line + i, src_col + j) == (dest_line, dest_col) {
                return true;
            }
        }
    }

    false
}
