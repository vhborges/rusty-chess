use crate::utils::constants::BOARD_SIZE;
use crate::utils::types::Board;
use crate::utils::Position;

pub const SYMBOLS: [char; 2] = ['\u{2654}', '\u{265A}'];

pub fn can_move(origin: Position, destination: Position, board: Board) -> bool {
    let (line, col) = (*origin.line(), *origin.col());

    // TODO check if the King will capture a oposite-color piece

    for i in -1..2_i8 {
        for j in -1..2_i8 {
            if i == 0 && j == 0 {
                // don't allow the piece to stay where it is
                continue;
            }
            if (line as i8 + i) < 0
                || (line as i8 + i) >= BOARD_SIZE as i8
                || (col as i8 + j) < 0
                || (col as i8 + j) >= BOARD_SIZE as i8
            {
                continue;
            }
            if (line as i8 + i, col as i8 + j)
                == (*destination.line() as i8, *destination.col() as i8)
            {
                return true;
            }
        }
    }

    false
}
