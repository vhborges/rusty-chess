use std::process::Command;

use crate::errors::MoveError;
use crate::io::{get_next_char, initial_positions};
use crate::pieces::{Piece, PieceType};
use crate::utils::constants::{BLANK_SQUARE, CAPTURE, COLUMNS, COL_RANGE, LINES, LINE_RANGE};
use crate::utils::types::{Board, Move};
use crate::utils::{ChessPosition, Color, Position};

pub struct GameState {
    board: Board,
    captured_white_pieces: Vec<Piece>,
    captured_black_pieces: Vec<Piece>,
    turn: Color,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Default::default(),
            captured_white_pieces: Vec::new(),
            captured_black_pieces: Vec::new(),
            turn: Color::White,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn captured_white_pieces(&self) -> &Vec<Piece> {
        &self.captured_white_pieces
    }

    pub fn captured_black_pieces(&self) -> &Vec<Piece> {
        &self.captured_black_pieces
    }

    pub fn add_piece(&mut self, piece: Piece) {
        let line = piece.position.line;
        let col = piece.position.col;

        self.board[line][col] = Some(piece);
    }

    fn parse_move(&self, str_move: String) -> Result<Move, MoveError> {
        let (origin, destination): (Position, Position);
        let dest_line: char;
        let mut dest_col = None;
        let (mut capture, check, checkmate) = (false, false, false);
        let mut disambiguation = None;

        let mut chars = str_move.chars();

        // First: Piece Type, Disambiguation (if Pawn and second = Capture)
        let first = chars
            .next()
            .ok_or(MoveError::InvalidMove("Empty input".to_owned()))?;
        let piece_type = first.try_into()?;
        if piece_type == PieceType::Pawn {
            dest_col = Some(first);
        }

        // Second: Disambiguation, Line, Column, Capture
        let mut next_char = chars.next().ok_or(MoveError::InvalidMove(
            "Missing second character".to_owned(),
        ))?;
        if next_char.is_digit(10) {
            if let Some(col) = dest_col {
                dest_line = next_char;
                destination = ChessPosition::new(dest_line, col).try_into()?;
                origin = self.find_piece_position(piece_type, destination, disambiguation)?;
                return Ok(Move::new(origin, destination));
            }
        }
        if next_char == CAPTURE {
            capture = true;
            if piece_type == PieceType::Pawn {
                disambiguation = Some(first);
            }
        } else if str_move.len() > 3
            && piece_type != PieceType::Pawn
            && (LINE_RANGE.contains(&next_char) || COL_RANGE.contains(&next_char))
        {
            disambiguation = Some(next_char);
        } else if next_char.is_lowercase() {
            dest_col = Some(next_char);
        } else {
            return Err(MoveError::InvalidCharacter(next_char));
        }

        // Third: Capture, Line, Column
        next_char = chars
            .next()
            .ok_or(MoveError::InvalidMove("Missing third character".to_owned()))?;
        if next_char == CAPTURE {
            capture = true;
        } else if next_char.is_digit(10) && piece_type != PieceType::Pawn {
            if let Some(col) = dest_col {
                dest_line = next_char;
                destination = ChessPosition::new(dest_line, col).try_into()?;
                origin = self.find_piece_position(piece_type, destination, disambiguation)?;
                return Ok(Move::new(origin, destination));
            }
            return Err(MoveError::InvalidMove(
                "Missing destination column".to_owned(),
            ));
        } else if next_char.is_lowercase() {
            dest_col = Some(next_char);
        } else {
            return Err(MoveError::InvalidCharacter(next_char));
        }

        // Fourth: Line (if capture), Column (if not Pawn and capture and disambiguation = Some)
        next_char = chars.next().ok_or(MoveError::InvalidMove(
            "Missing fourth character".to_owned(),
        ))?;
        if next_char.is_digit(10) && capture {
            if let Some(col) = dest_col {
                dest_line = next_char;
                destination = ChessPosition::new(dest_line, col).try_into()?;
                origin = self.find_piece_position(piece_type, destination, disambiguation)?;
                return Ok(Move::new(origin, destination));
            }
            return Err(MoveError::InvalidMove(
                "Missing destination column".to_owned(),
            ));
        } else if next_char.is_lowercase() {
            dest_col = Some(next_char);
        } else {
            return Err(MoveError::InvalidCharacter(next_char));
        }

        //Fifth: Line
        next_char = chars
            .next()
            .ok_or(MoveError::InvalidMove("Missing fifth character".to_owned()))?;
        if !next_char.is_digit(10) {
            return Err(MoveError::InvalidCharacter(next_char));
        }
        if let Some(col) = dest_col {
            dest_line = next_char;
            destination = ChessPosition::new(dest_line, col).try_into()?;
            origin = self.find_piece_position(piece_type, destination, disambiguation)?;
            return Ok(Move::new(origin, destination));
        }
        return Err(MoveError::InvalidMove(
            "Missing destination column".to_owned(),
        ));
    }

    fn find_piece_position(
        &self,
        piece_type: PieceType,
        destination: Position,
        disambiguation: Option<char>,
    ) -> Result<Position, MoveError> {
        let mut matching_pieces = Vec::new();
        for line in self.board {
            for opt_piece in line {
                if let Some(piece) = opt_piece {
                    if piece.piece_type != piece_type || piece.color != self.turn {
                        continue;
                    }
                    if piece.can_move(self.board, destination) {
                        matching_pieces.push(piece);
                    }
                }
            }
        }

        if matching_pieces.is_empty() {
            return Err(MoveError::InvalidMove(
                "No piece available for this move".to_owned(),
            ));
        }
        if matching_pieces.len() > 1 {
            if disambiguation.is_none() {
                return Err(MoveError::InvalidMove(
                    "More than one piece available for this move".to_owned(),
                ));
            }
            for i in 1..matching_pieces.len() {
                let piece = matching_pieces[i];
                let chess_pos: ChessPosition = piece.position.try_into()?;
                if disambiguation.unwrap() != chess_pos.line
                    && disambiguation.unwrap() != chess_pos.col
                {
                    matching_pieces.remove(i);
                }
            }
            if matching_pieces.len() != 1 {
                return Err(MoveError::InvalidMove(
                    "More than one piece available for this move".to_owned(),
                ));
            }
        }

        Ok(matching_pieces[0].position)
    }

    pub fn move_piece(&mut self, str_move: String) -> Result<(), MoveError> {
        let next_move = self.parse_move(str_move)?;

        let source_line = next_move.source.line;
        let source_col = next_move.source.col;

        let dest_line = next_move.destination.line;
        let dest_col = next_move.destination.col;

        let dest_piece = self.board[dest_line][dest_col];

        if let Some(captured_piece) = dest_piece {
            match captured_piece.color {
                Color::White => self.captured_white_pieces.push(captured_piece),
                Color::Black => self.captured_black_pieces.push(captured_piece),
            }
        }

        let mut source_piece = self.board[source_line][source_col];
        source_piece.as_mut().unwrap().position = Position::new(dest_line, dest_col);

        self.board[source_line][source_col] = None;
        self.board[dest_line][dest_col] = source_piece;

        self.turn.flip();

        Ok(())
    }

    pub fn initialize(&mut self) {
        for wrapped_line in initial_positions() {
            let line = wrapped_line.expect("Error reading file line");
            let mut chars = line.chars();

            let piece_color: Color = get_next_char(&line, &mut chars)
                .try_into()
                .expect(format!("Could not parse color character from line {}", line).as_str());

            let piece_type: PieceType = get_next_char(&line, &mut chars)
                .try_into()
                .expect(format!("Could not parse piece character from line {}", line).as_str());

            let chess_col = get_next_char(&line, &mut chars);

            let chess_line = get_next_char(&line, &mut chars);

            let piece_position = ChessPosition::new(chess_line, chess_col).try_into().expect(
                format!(
                    "Could not convert ChessPosition {}{} to Position",
                    chess_col, chess_line
                )
                .as_str(),
            );

            self.add_piece(Piece::new(piece_type, piece_color, piece_position))
        }
    }

    pub fn print(&self) {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");

        for (line, line_chess) in self.board().iter().zip(LINES.iter()) {
            print!("{} ", line_chess);
            for opt_piece in line {
                match opt_piece {
                    Some(piece) => print!("{} ", piece),
                    None => print!("{} ", BLANK_SQUARE),
                }
            }
            println!();
        }

        print!("  ");

        for col_chess in COLUMNS {
            print!("{} ", col_chess);
        }

        println!();

        for piece in self.captured_white_pieces() {
            print!("{} ", piece)
        }

        println!();

        for piece in self.captured_black_pieces() {
            print!("{} ", piece)
        }

        println!()
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::MoveError;
    use crate::utils::{Color, Position};
    use crate::utils::types::Move;

    use super::GameState;

    #[test]
    fn test_move_piece() {
        let mut game_state = GameState::new();
        game_state.initialize();

        assert_eq!(game_state.turn, Color::White);
        make_and_assert_move(&mut game_state, "e3", Position::new(6, 4), Position::new(5, 4));

        assert_eq!(game_state.turn, Color::Black);
        make_and_assert_move(&mut game_state, "e6", Position::new(1, 4), Position::new(2, 4));

        assert_eq!(game_state.turn, Color::White);
        make_and_assert_move(&mut game_state, "Bb5", Position::new(7, 5), Position::new(3, 1));

        assert_eq!(game_state.turn, Color::Black);
        make_and_assert_move(&mut game_state, "Nf6", Position::new(0, 6), Position::new(2, 5));

        assert_eq!(game_state.turn, Color::White);
        make_and_assert_move(&mut game_state, "Bxd7", Position::new(3, 1), Position::new(1, 3));

        assert_eq!(game_state.turn, Color::Black);
        make_and_assert_move(&mut game_state, "Qxd7", Position::new(0, 3), Position::new(1, 3));

        assert_eq!(game_state.turn, Color::White);
        make_and_assert_move(&mut game_state, "d4", Position::new(6, 3), Position::new(4, 3));

        assert_eq!(game_state.turn, Color::Black);
        make_and_assert_move(&mut game_state, "Bc5", Position::new(0, 5), Position::new(3, 2));

        assert_eq!(game_state.turn, Color::White);
        make_and_assert_move(&mut game_state, "dxc5", Position::new(4, 3), Position::new(3, 2));

        assert_eq!(game_state.turn, Color::Black);
        make_and_assert_move(&mut game_state, "Na6", Position::new(0, 1), Position::new(2, 0));

        assert_eq!(game_state.turn, Color::White);
        make_and_assert_move(&mut game_state, "Nc3", Position::new(7, 1), Position::new(5, 2));

        assert_eq!(game_state.turn, Color::Black);
        make_and_assert_move(&mut game_state, "Ne4", Position::new(2, 5), Position::new(4, 4));

        assert_eq!(game_state.turn, Color::White);
        make_and_assert_move(&mut game_state, "Qxd7+", Position::new(7, 3), Position::new(1, 3));

        assert_eq!(game_state.turn, Color::Black);
        make_and_assert_move(&mut game_state, "Kxd7", Position::new(0, 4), Position::new(1, 3));

        assert_eq!(game_state.turn, Color::White);
        make_and_assert_move(&mut game_state, "h4", Position::new(6, 7), Position::new(4, 7));

        assert_eq!(game_state.turn, Color::Black);
        make_and_assert_move(&mut game_state, "Naxc5", Position::new(2, 0), Position::new(3, 2));
    }

    fn make_and_assert_move(game_state: &mut GameState, str_move: &str, source: Position, destination: Position) {
        let origin_piece = game_state.board[source.line][source.col];
        assert!(origin_piece.is_some());

        let result = game_state.move_piece(str_move.to_owned());
        // TODO assert result is Ok
        if let Err(error) = result {
            println!("Move error: {}", error);
            panic!();
        }

        let dest_piece = game_state.board[destination.line][destination.col];
        assert!(dest_piece.is_some());
        assert_eq!(origin_piece.unwrap().piece_type, dest_piece.unwrap().piece_type);
        assert_eq!(origin_piece.unwrap().color, dest_piece.unwrap().color);
    }
}