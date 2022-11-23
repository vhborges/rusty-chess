use crate::{
    errors::{ChessPositionError, MoveError, PgnError},
    game_state::GameState,
    pieces::PieceType,
};

use super::{
    constants::{CAPTURE, COL_RANGE, LINE_RANGE},
    types::Move,
    ChessPosition, Position,
};

pub fn parse_move(game_state: &GameState, str_move: String) -> Result<Move, MoveError> {
    let (origin, destination): (Position, Position);
    let dest_line: char;
    let mut dest_col = None;
    let (mut capture, check, checkmate) = (false, false, false);
    let mut disambiguation = None;

    let mut chars = str_move.chars();

    // TODO create better state machine

    // First: Piece Type, Disambiguation (if Pawn and second = Capture)
    let first = chars.next().ok_or(PgnError::EmptyInput)?;
    let piece_type = first.try_into()?;
    if piece_type == PieceType::Pawn {
        dest_col = Some(first);
    }

    // Second: Disambiguation, Line, Column, Capture
    let mut next_char = chars.next().ok_or(PgnError::MissingCharacter("second"))?;
    if next_char.is_digit(10) {
        let Some(col) = dest_col else {
            return Err(ChessPositionError::MissingDestinationColumn.into());
        };
        dest_line = next_char;
        destination = ChessPosition::new(dest_line, col).try_into()?;
        origin =
            game_state.find_piece_position(piece_type, destination, disambiguation, capture)?;
        return Ok(Move::new(origin, destination));
    }
    if next_char == CAPTURE {
        capture = true;
        if piece_type == PieceType::Pawn {
            disambiguation = Some(first);
        }
    }
    else if str_move.len() > 3
        && piece_type != PieceType::Pawn
        && (LINE_RANGE.contains(&next_char) || COL_RANGE.contains(&next_char))
    {
        disambiguation = Some(next_char);
    }
    else if next_char.is_lowercase() {
        dest_col = Some(next_char);
    }
    else {
        return Err(PgnError::InvalidCharacter(next_char).into());
    }

    // Third: Capture, Line, Column
    next_char = chars.next().ok_or(PgnError::MissingCharacter("third"))?;
    if next_char == CAPTURE {
        capture = true;
    }
    else if next_char.is_digit(10) && piece_type != PieceType::Pawn {
        let Some(col) = dest_col else {
            return Err(ChessPositionError::MissingDestinationColumn.into());
        };
        dest_line = next_char;
        destination = ChessPosition::new(dest_line, col).try_into()?;
        origin =
            game_state.find_piece_position(piece_type, destination, disambiguation, capture)?;
        return Ok(Move::new(origin, destination));
    }
    else if next_char.is_lowercase() {
        dest_col = Some(next_char);
    }
    else {
        return Err(PgnError::InvalidCharacter(next_char).into());
    }

    // Fourth: Line (if capture or disambiguation), Column (if not Pawn and capture and disambiguation = Some)
    next_char = chars.next().ok_or(PgnError::MissingCharacter("fourth"))?;
    if next_char.is_digit(10) && (capture || disambiguation.is_some()) {
        let Some(col) = dest_col else {
            return Err(ChessPositionError::MissingDestinationColumn.into());
        };
        dest_line = next_char;
        destination = ChessPosition::new(dest_line, col).try_into()?;
        origin =
            game_state.find_piece_position(piece_type, destination, disambiguation, capture)?;
        return Ok(Move::new(origin, destination));
    }
    if next_char.is_lowercase() {
        dest_col = Some(next_char);
    }
    else {
        return Err(PgnError::InvalidCharacter(next_char).into());
    }

    //Fifth: Line
    next_char = chars.next().ok_or(PgnError::MissingCharacter("fifth"))?;
    if !next_char.is_digit(10) {
        return Err(ChessPositionError::MissingDestinationLine.into());
    }
    let Some(col) = dest_col else {
        return Err(ChessPositionError::MissingDestinationColumn.into());
    };
    dest_line = next_char;
    destination = ChessPosition::new(dest_line, col).try_into()?;
    origin = game_state.find_piece_position(piece_type, destination, disambiguation, capture)?;
    return Ok(Move::new(origin, destination));
}
