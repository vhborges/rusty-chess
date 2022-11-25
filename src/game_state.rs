use crate::errors::MoveError;
use crate::io::{get_next_char, initial_positions};
use crate::pieces::{Piece, PieceType};
use crate::utils::{pgn_utils::parse_move, types::Board, ChessPosition, Color, Position};

pub struct GameState {
    pub board: Board,
    pub captured_white_pieces: Vec<Piece>,
    pub captured_black_pieces: Vec<Piece>,
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

    pub fn add_piece(&mut self, piece: Piece) {
        let line = piece.position.line;
        let col = piece.position.col;

        self.board[line][col] = Some(piece);
    }

    pub fn find_piece_position(
        &self,
        piece_type: PieceType,
        destination: Position,
        disambiguation: Option<char>,
        capture: bool,
    ) -> Result<Position, MoveError> {
        let mut matching_pieces = Vec::new();
        for line in self.board {
            for opt_piece in line {
                if let Some(piece) = opt_piece {
                    if piece.piece_type != piece_type || piece.color != self.turn {
                        continue;
                    }
                    if piece.can_move(self.board, destination, capture)? {
                        matching_pieces.push(piece);
                    }
                }
            }
        }

        if matching_pieces.is_empty() {
            return Err(MoveError::NoPieceAvailable);
        }
        if matching_pieces.len() > 1 {
            if disambiguation.is_none() {
                return Err(MoveError::MoreThanOnePieceAvailable);
            }
            matching_pieces.retain(|piece| -> bool {
                let chess_pos: ChessPosition = piece
                    .position
                    .try_into()
                    .expect("Internal error 02: Invalid piece position");
                disambiguation.unwrap() == chess_pos.line
                    || disambiguation.unwrap() == chess_pos.col
            });
            if matching_pieces.len() != 1 {
                return Err(MoveError::MoreThanOnePieceAvailable);
            }
        }

        Ok(matching_pieces[0].position)
    }

    pub fn move_piece(&mut self, str_move: String) -> Result<(), MoveError> {
        let next_move = parse_move(&self, str_move)?;

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
}

#[cfg(test)]
mod tests {
    use crate::{
        errors::{ChessPositionError, MoveError, PgnError},
        utils::{Color, Position},
    };

    use super::GameState;

    #[test]
    fn test_move_piece() -> Result<(), MoveError> {
        let mut game_state = GameState::new();
        game_state.initialize();

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "e3",
            Position::new(6, 4),
            Position::new(5, 4),
        )?;

        assert_eq!(game_state.turn, Color::Black);
        make_and_validate_move(
            &mut game_state,
            "e6",
            Position::new(1, 4),
            Position::new(2, 4),
        )?;

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "Bb5",
            Position::new(7, 5),
            Position::new(3, 1),
        )?;

        assert_eq!(game_state.turn, Color::Black);
        make_and_validate_move(
            &mut game_state,
            "Nf6",
            Position::new(0, 6),
            Position::new(2, 5),
        )?;

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "Bxd7",
            Position::new(3, 1),
            Position::new(1, 3),
        )?;

        assert_eq!(game_state.turn, Color::Black);
        make_and_validate_move(
            &mut game_state,
            "Qxd7",
            Position::new(0, 3),
            Position::new(1, 3),
        )?;

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "d4",
            Position::new(6, 3),
            Position::new(4, 3),
        )?;

        assert_eq!(game_state.turn, Color::Black);
        make_and_validate_move(
            &mut game_state,
            "Bc5",
            Position::new(0, 5),
            Position::new(3, 2),
        )?;

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "dxc5",
            Position::new(4, 3),
            Position::new(3, 2),
        )?;

        assert_eq!(game_state.turn, Color::Black);
        make_and_validate_move(
            &mut game_state,
            "Na6",
            Position::new(0, 1),
            Position::new(2, 0),
        )?;

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "Nc3",
            Position::new(7, 1),
            Position::new(5, 2),
        )?;

        assert_eq!(game_state.turn, Color::Black);
        make_and_validate_move(
            &mut game_state,
            "Ne4",
            Position::new(2, 5),
            Position::new(4, 4),
        )?;

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "Qxd7+",
            Position::new(7, 3),
            Position::new(1, 3),
        )?;

        assert_eq!(game_state.turn, Color::Black);
        make_and_validate_move(
            &mut game_state,
            "Kxd7",
            Position::new(0, 4),
            Position::new(1, 3),
        )?;

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "h4",
            Position::new(6, 7),
            Position::new(4, 7),
        )?;

        assert_eq!(game_state.turn, Color::Black);
        make_and_validate_move(
            &mut game_state,
            "Naxc5",
            Position::new(2, 0),
            Position::new(3, 2),
        )?;

        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "Rh2",
            Position::new(7, 7),
            Position::new(6, 7),
        )?;

        Ok(())
    }

    #[test]
    fn test_invalid_move() {
        let mut game_state = GameState::new();
        game_state.initialize();

        let mut result = game_state.move_piece("Kd5".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::NoPieceAvailable);

        game_state
            .move_piece("e4".to_owned())
            .expect("Something's wrong: e4 is not a invalid move!");
        game_state
            .move_piece("c5".to_owned())
            .expect("Something's wrong: c5 is not a invalid move!");
        result = game_state.move_piece("exc5".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::NoPieceAvailable);

        game_state
            .move_piece("d4".to_owned())
            .expect("Something's wrong: d4 is not a invalid move!");
        game_state
            .move_piece("cxd4".to_owned())
            .expect("Something's wrong: cxd4 is not a invalid move!");
        game_state
            .move_piece("Nf3".to_owned())
            .expect("Something's wrong: Nf3 is not a invalid move!");
        game_state
            .move_piece("e5".to_owned())
            .expect("Something's wrong: e5 is not a invalid move!");
        result = game_state.move_piece("Nd2".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::MoreThanOnePieceAvailable);

        game_state
            .move_piece("Nbd2".to_owned())
            .expect("Something's wrong: Nbd2 is not a invalid move!");
        game_state
            .move_piece("Bd6".to_owned())
            .expect("Something's wrong: Bd6 is not a invalid move!");
        game_state
            .move_piece("Nxd4".to_owned())
            .expect("Something's wrong: Nxd4 is not a invalid move!");
        game_state
            .move_piece("Nc6".to_owned())
            .expect("Something's wrong: Nc6 is not a invalid move!");
        result = game_state.move_piece("Ndb3".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::MoreThanOnePieceAvailable);
    }

    #[test]
    fn test_invalid_capture() {
        let mut game_state = GameState::new();
        game_state.initialize();

        let result = game_state.move_piece("exd3".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MoveError::InvalidCapture("Destination square is empty")
        )
    }

    #[test]
    fn test_invalid_pgn_string() {
        let mut game_state = GameState::new();
        game_state.initialize();

        let mut result = game_state.move_piece("e".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PgnError::MissingCharacter("second").into()
        );

        result = game_state.move_piece("eK".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('K').into());

        result = game_state.move_piece("Kx5".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ChessPositionError::MissingDestinationColumn.into()
        );

        result = game_state.move_piece("KxI".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('I').into());

        result = game_state.move_piece("Kxc".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PgnError::MissingCharacter("fourth").into()
        );

        result = game_state.move_piece("Kxx7".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ChessPositionError::MissingDestinationColumn.into()
        );

        result = game_state.move_piece("KxdL".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('L').into());

        result = game_state.move_piece("KdxcM".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ChessPositionError::MissingDestinationLine.into()
        );

        result = game_state.move_piece("Le5".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PgnError::InvalidPiece('L').into());

        game_state
            .move_piece("e4".to_owned())
            .expect("Something's wrong: e4 is not a invalid move!");
        game_state
            .move_piece("d5".to_owned())
            .expect("Something's wrong: d5 is not a invalid move!");
        game_state
            .move_piece("Nc3".to_owned())
            .expect("Something's wrong: Nc3 is not a invalid move!");
        game_state
            .move_piece("Nf6".to_owned())
            .expect("Something's wrong: Nf6 is not a invalid move!");
        result = game_state.move_piece("Nd5".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PgnError::MissingCaptureCharacter.into());
    }

    fn make_and_validate_move(
        game_state: &mut GameState,
        str_move: &str,
        source: Position,
        destination: Position,
    ) -> Result<(), MoveError> {
        let origin_piece = game_state.board[source.line][source.col];
        assert!(origin_piece.is_some());

        game_state.move_piece(str_move.to_owned())?;

        let dest_piece = game_state.board[destination.line][destination.col];
        assert!(dest_piece.is_some());
        assert_eq!(
            origin_piece.unwrap().piece_type,
            dest_piece.unwrap().piece_type
        );
        assert_eq!(origin_piece.unwrap().color, dest_piece.unwrap().color);

        Ok(())
    }
}
