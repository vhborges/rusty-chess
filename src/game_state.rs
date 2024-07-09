use crate::errors::MoveError;
use crate::io::{get_next_char, initial_positions};
use crate::pieces::{Piece, PieceType};
use crate::utils::{pgn::pgn_utils::parse_move, types::Board, ChessPosition, Color, Position};
use crate::utils::types::Move;

pub struct GameState {
    pub board: Board,
    pub captured_white_pieces: Vec<Piece>,
    pub captured_black_pieces: Vec<Piece>,
    turn: Color,
    white_king_position: Position,
    black_king_position: Position,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Default::default(),
            captured_white_pieces: Vec::new(),
            captured_black_pieces: Vec::new(),
            turn: Color::White,
            white_king_position: Default::default(),
            black_king_position: Default::default(),
        }
    }

    pub fn add_piece(&mut self, piece: Piece, pos: Position) {
        self.board[pos.line][pos.col] = Some(piece);
    }

    pub fn find_piece_position(
        &self,
        piece_type: PieceType,
        destination: Position,
        opt_disambiguation: Option<char>,
        capture: bool,
    ) -> Result<Position, MoveError> {
        let mut matching_positions = Vec::new();
        for (line_index, line) in self.board.iter().enumerate() {
            for (col_index, opt_piece) in line.iter().enumerate() {
                if let Some(piece) = opt_piece {
                    let origin = Position::new(line_index, col_index);
                    if self.piece_matches(piece, piece_type, origin, destination, capture)?
                    {
                        matching_positions.push(origin);
                    }
                }
            }
        }

        if matching_positions.is_empty() {
            return Err(MoveError::NoPieceAvailable);
        }
        if matching_positions.len() > 1 {
            let Some(disambiguation) = opt_disambiguation else {
                return Err(MoveError::MoreThanOnePieceAvailable);
            };

            matching_positions.retain(|pos| -> bool {
                let chess_pos: ChessPosition = (*pos)
                    .try_into()
                    .expect("Internal error 02: Invalid piece position");

                return disambiguation == chess_pos.line || disambiguation == chess_pos.col;
            });

            if matching_positions.len() != 1 {
                return Err(MoveError::MoreThanOnePieceAvailable);
            }
        }

        Ok(matching_positions[0])
    }

    fn piece_matches(&self, piece: &Piece, piece_type: PieceType, origin: Position, destination: Position, capture: bool) -> Result<bool, MoveError> {
        Ok(piece.piece_type == piece_type
            && piece.color == self.turn
            && piece.can_move(&self.board, origin, destination, capture)?)
    }

    pub fn move_piece(&mut self, str_move: String) -> Result<(), MoveError> {
        let next_move = parse_move(&self, str_move)?;

        let source_line = next_move.source.line;
        let source_col = next_move.source.col;

        let dest_line = next_move.destination.line;
        let dest_col = next_move.destination.col;

        self.verify_king_in_check(&next_move, dest_line, dest_col)?;

        self.update_king_position(&next_move, source_line, source_col);

        self.update_captured_pieces_list(dest_line, dest_col);

        Self::perform_move(&next_move, &mut self.board);

        self.turn.flip();

        Ok(())
    }

    fn verify_king_in_check(&mut self, next_move: &Move, dest_line: usize, dest_col: usize) -> Result<(), MoveError> {
        let mut temporary_board = self.board.clone();
        Self::perform_move(&next_move, &mut temporary_board);

        let king_pos = self.get_king_pos(dest_line, dest_col, temporary_board);

        if self.is_king_in_check(&temporary_board, king_pos) {
            return Err(MoveError::KingWouldBeInCheck);
        }
        
        Ok(())
    }

    fn update_captured_pieces_list(&mut self, dest_line: usize, dest_col: usize) {
        let dest_piece = self.board[dest_line][dest_col];
        if let Some(captured_piece) = dest_piece {
            match captured_piece.color {
                Color::White => self.captured_white_pieces.push(captured_piece),
                Color::Black => self.captured_black_pieces.push(captured_piece),
            }
        }
    }

    fn update_king_position(&mut self, next_move: &Move, source_line: usize, source_col: usize) {
        let source_piece = self.board[source_line][source_col].unwrap();
        if source_piece.piece_type == PieceType::King {
            match self.turn {
                Color::White => self.white_king_position = next_move.destination,
                Color::Black => self.black_king_position = next_move.destination,
            }
        }
    }

    fn get_king_pos(&self, dest_line: usize, dest_col: usize, temporary_board: [[Option<Piece>; 8]; 8]) -> Position {
        let king_pos =
            if temporary_board[dest_line][dest_col].unwrap().piece_type == PieceType::King {
                Position::new(dest_line, dest_col)
            } else {
                match self.turn {
                    Color::White => self.white_king_position,
                    Color::Black => self.black_king_position,
                }
            };
        king_pos
    }

    fn perform_move(_move: &Move, temporary_board: &mut Board) {
        temporary_board[_move.destination.line][_move.destination.col] = temporary_board[_move.source.line][_move.source.col];
        temporary_board[_move.source.line][_move.source.col] = None;
    }

    fn is_king_in_check(&self, board: &Board, king_pos: Position) -> bool {
        for (line_index, line) in board.iter().enumerate() {
            for (col_index, opt_piece) in line.iter().enumerate() {
                if let Some(piece) = opt_piece {
                    if piece.color == self.turn {
                        continue
                    }

                    let piece_pos = Position::new(line_index, col_index);

                    if piece.attacks(board, piece_pos, king_pos) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn initialize(&mut self) {
        for wrapped_line in initial_positions() {
            let line = wrapped_line.expect("Error reading file line");
            let mut chars = line.chars();

            let piece_color: Color = get_next_char(&line, &mut chars).try_into().expect(&format!(
                "Could not parse color character from line {}",
                line
            ));

            let piece_type: PieceType = get_next_char(&line, &mut chars).try_into().expect(
                &format!("Could not parse piece character from line {}", line),
            );

            let chess_col = get_next_char(&line, &mut chars);

            let chess_line = get_next_char(&line, &mut chars);

            let piece_position =
                ChessPosition::new(chess_line, chess_col)
                    .try_into()
                    .expect(&format!(
                        "Could not convert ChessPosition {}{} to Position",
                        chess_col, chess_line
                    ));
            
            if piece_type == PieceType::King {
                match piece_color {
                    Color::White => self.white_king_position = piece_position,
                    Color::Black => self.black_king_position = piece_position,
                }
            }

            self.add_piece(Piece::new(piece_type, piece_color), piece_position);
        }
    }
}

// TODO study the possibility of moving these tests into a dedicated module
#[cfg(test)]
mod tests {
    use crate::{
        errors::{ChessPositionError, MoveError, PgnError},
        utils::{Color, Position},
    };
    use crate::io::ui::print_game;
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

        print_game(&game_state);
        
        assert_eq!(game_state.turn, Color::White);
        make_and_validate_move(
            &mut game_state,
            "Bxd7",
            Position::new(3, 1),
            Position::new(1, 3),
        )?;
        
        print_game(&game_state);

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

    macro_rules! setup_board {
        ( $game_state:expr, $( $x:expr ),* ) => {
            {
                $(
                    $game_state.move_piece($x.to_owned())
                        .expect(format!("Something's wrong, {} is not a invalid move!", $x).as_str());
                )*
            }
        };
    }

    #[test]
    fn test_invalid_move() {
        let mut game_state = GameState::new();
        game_state.initialize();

        // TODO segregate below tests into multiple functions

        let mut result = game_state.move_piece("Kd5".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::NoPieceAvailable);

        setup_board!(game_state, "e4", "c5");
        result = game_state.move_piece("exc5".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::NoPieceAvailable);

        setup_board!(game_state, "d4", "cxd4", "Nf3", "e5");
        result = game_state.move_piece("Nd2".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::MoreThanOnePieceAvailable);

        setup_board!(game_state, "Nbd2", "Bd6", "Nxd4", "Nc6");
        result = game_state.move_piece("Ndb3".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::MoreThanOnePieceAvailable);
    }

    #[test]
    fn test_square_occupied_error() {
        let mut game_state = GameState::new();
        game_state.initialize();

        let result = game_state.move_piece("Ke2".to_owned());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoveError::SquareOccupied);
    }

    #[test]
    fn test_invalid_capture() {
        let mut game_state = GameState::new();
        game_state.initialize();

        // TODO segregate below tests into multiple functions

        let result = game_state.move_piece("exd3".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MoveError::InvalidCapture("Destination square is empty")
        );

        let result = game_state.move_piece("Kxe2".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MoveError::InvalidCapture("Cannot capture a piece of the same color")
        );
    }

    #[test]
    fn test_invalid_pgn_string() {
        let mut game_state = GameState::new();
        game_state.initialize();

        // TODO segregate below tests into multiple functions

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

        setup_board!(game_state, "e4", "d5", "Nc3", "Nf6");
        result = game_state.move_piece("Nd5".to_owned());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PgnError::MissingCaptureCharacter.into()
        );
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
