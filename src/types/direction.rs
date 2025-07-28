use crate::types::PositionI8;

pub struct Direction {
    pub horizontal: i8,
    pub vertical: i8,
}

impl Direction {
    pub fn new(horizontal: i8, vertical: i8) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn from_position_i8(src: PositionI8, dest: PositionI8) -> Self {
        // Avoid division by zero
        let horizontal_direction = if (dest.col - src.col) != 0 {
            (dest.col - src.col) / (dest.col - src.col).abs()
        }
        else {
            0
        };

        let vertical_direction = if (dest.line - src.line) != 0 {
            (dest.line - src.line) / (dest.line - src.line).abs()
        }
        else {
            0
        };

        Direction::new(horizontal_direction, vertical_direction)
    }
}
