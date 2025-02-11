// TODO add and implement support for the CHECK character (+)

pub const BLANK_SQUARE: char = '_';

pub const CAPTURE: char = 'x';
pub const QUEEN_SIDE_CASTLING: &str = "O-O-O";

pub const WHITE_CASTLING_LINE: usize = 7;
pub const BLACK_CASTLING_LINE: usize = 0;

pub const INTERNAL_ERROR_01: &str = "Internal error 01: Invalid piece position";
pub const INTERNAL_ERROR_02: &str =
    "Internal error 02: piece.attacks() should not return error when capture=false";
pub const INTERNAL_ERROR_03: &str =
    "Internal error 03: constant castling PGN constant should have 5 characters";
pub const INTERNAL_ERROR_04: &str = "Internal error 04: invalid function call: a PieceType of the None kind cannot be used in this function";
pub const INTERNAL_ERROR_05: &str = "Internal error 05: invalid enum variant for this state";
