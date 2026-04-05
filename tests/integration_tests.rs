use chess::GameState;
use chess::errors::{ChessPositionError, MoveError, PgnError};
use chess::movement::Position;
use std::mem::discriminant;

macro_rules! setup_board {
        ( $game_state:expr, $( $x:expr ),* ) => {
            {
                $(
                    $game_state.handle_move($x)
                        .expect(format!("Something's wrong, {} is not an invalid move!", $x).as_str());
                )*
            }
        };
    }

pub fn setup() -> GameState {
    let mut game_state = GameState::new();
    game_state.initialize(None);
    game_state
}

pub fn setup_with_positions(file: &str) -> GameState {
    let mut game_state = GameState::new();
    game_state.initialize(Some(file));
    game_state
}

fn assert_failed_move_preserves_state(
    game_state: &mut GameState,
    pgn_move: &str,
    expected_error: MoveError,
) {
    let snapshot = game_state.clone();

    let result = game_state.handle_move(pgn_move);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), expected_error);
    assert_eq!(*game_state, snapshot);
}

fn make_and_validate_move(
    game_state: &mut GameState,
    str_move: &str,
    source: Position,
    destination: Position,
) -> Result<(), MoveError> {
    let origin_piece = game_state.get_piece(source);
    assert!(origin_piece.is_some());

    game_state.handle_move(str_move)?;

    let dest_piece = game_state.get_piece(destination);
    assert!(dest_piece.is_some());

    assert_eq!(
        discriminant(&origin_piece.unwrap().piece_type),
        discriminant(&dest_piece.unwrap().piece_type)
    );
    assert_eq!(origin_piece.unwrap().color, dest_piece.unwrap().color);
    assert!(game_state.get_piece(source).is_none());

    Ok(())
}

#[test]
fn test_successful_game() -> Result<(), MoveError> {
    let mut game_state = setup();

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "e3",
        Position::new(6, 4),
        Position::new(5, 4),
    )?;

    assert!(game_state.is_black_turn());
    make_and_validate_move(
        &mut game_state,
        "e6",
        Position::new(1, 4),
        Position::new(2, 4),
    )?;

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "Bb5",
        Position::new(7, 5),
        Position::new(3, 1),
    )?;

    assert!(game_state.is_black_turn());
    make_and_validate_move(
        &mut game_state,
        "Nf6",
        Position::new(0, 6),
        Position::new(2, 5),
    )?;

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "Bxd7",
        Position::new(3, 1),
        Position::new(1, 3),
    )?;

    assert!(game_state.is_black_turn());
    make_and_validate_move(
        &mut game_state,
        "Qxd7",
        Position::new(0, 3),
        Position::new(1, 3),
    )?;

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "d4",
        Position::new(6, 3),
        Position::new(4, 3),
    )?;

    assert!(game_state.is_black_turn());
    make_and_validate_move(
        &mut game_state,
        "Bc5",
        Position::new(0, 5),
        Position::new(3, 2),
    )?;

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "dxc5",
        Position::new(4, 3),
        Position::new(3, 2),
    )?;

    assert!(game_state.is_black_turn());
    make_and_validate_move(
        &mut game_state,
        "Na6",
        Position::new(0, 1),
        Position::new(2, 0),
    )?;

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "Nc3",
        Position::new(7, 1),
        Position::new(5, 2),
    )?;

    assert!(game_state.is_black_turn());
    make_and_validate_move(
        &mut game_state,
        "Ne4",
        Position::new(2, 5),
        Position::new(4, 4),
    )?;

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "Qxd7+",
        Position::new(7, 3),
        Position::new(1, 3),
    )?;

    assert!(game_state.is_black_turn());
    make_and_validate_move(
        &mut game_state,
        "Kxd7",
        Position::new(0, 4),
        Position::new(1, 3),
    )?;

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "h4",
        Position::new(6, 7),
        Position::new(4, 7),
    )?;

    assert!(game_state.is_black_turn());
    make_and_validate_move(
        &mut game_state,
        "Naxc5",
        Position::new(2, 0),
        Position::new(3, 2),
    )?;

    assert!(game_state.is_white_turn());
    make_and_validate_move(
        &mut game_state,
        "Rh2",
        Position::new(7, 7),
        Position::new(6, 7),
    )?;

    Ok(())
}

#[test]
fn test_no_piece_available_error() {
    let mut game_state = setup();

    let result = game_state.handle_move("Kd5");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::NoPieceAvailable);

    setup_board!(game_state, "e4", "c5");

    let result = game_state.handle_move("exc5");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::NoPieceAvailable);
}

#[test]
fn test_more_than_one_piece_available_error() {
    let mut game_state = setup();
    setup_board!(game_state, "e4", "c5", "d4", "cxd4", "Nf3", "e5");

    let result = game_state.handle_move("Nd2");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::MoreThanOnePieceAvailable);

    setup_board!(game_state, "Nbd2", "Bd6", "Nxd4", "Nc6");

    let result = game_state.handle_move("Ndb3");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::MoreThanOnePieceAvailable);
}

#[test]
fn test_square_occupied_error() {
    let mut game_state = setup();

    assert_failed_move_preserves_state(&mut game_state, "Ke2", MoveError::SquareOccupied);
}

#[test]
fn test_destination_square_empty_error() {
    let mut game_state = setup();

    assert_failed_move_preserves_state(
        &mut game_state,
        "exd3",
        MoveError::InvalidCapture("Destination square is empty"),
    );
}

#[test]
fn test_same_color_piece_capture_error() {
    let mut game_state = setup();

    assert_failed_move_preserves_state(
        &mut game_state,
        "Kxe2",
        MoveError::InvalidCapture("Cannot capture a piece of the same color"),
    );
}

#[test]
fn test_missing_second_character_error() {
    let mut game_state = setup();

    assert_failed_move_preserves_state(
        &mut game_state,
        "e",
        PgnError::MissingCharacter("second").into(),
    );

    let result = game_state.handle_move("e");

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCharacter("second").into()
    );
}

#[test]
fn test_invalid_character_error_on_destination() {
    let mut game_state = setup();

    assert_failed_move_preserves_state(
        &mut game_state,
        "eK",
        PgnError::InvalidCharacter('K').into(),
    );
}

#[test]
fn test_invalid_character_error_on_capture_destination() {
    let mut game_state = setup();

    assert_failed_move_preserves_state(
        &mut game_state,
        "KxI",
        PgnError::InvalidCharacter('I').into(),
    );
}

#[test]
fn test_invalid_character_error_on_fifth_character() {
    let mut game_state = setup();

    assert_failed_move_preserves_state(
        &mut game_state,
        "KxdL",
        PgnError::InvalidCharacter('L').into(),
    );
}

#[test]
fn test_missing_destination_column_error() {
    let mut game_state = setup();

    let result = game_state.handle_move("Kx5");

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ChessPositionError::MissingDestinationColumn.into()
    );

    let result = game_state.handle_move("Kxx7");

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ChessPositionError::MissingDestinationColumn.into()
    );
}

#[test]
fn test_missing_fourth_character_error() {
    let mut game_state = setup();

    let result = game_state.handle_move("Kxc");

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCharacter("fourth").into()
    );
}

#[test]
fn test_missing_destination_line_error() {
    let mut game_state = setup();

    let result = game_state.handle_move("KdxcM");

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ChessPositionError::MissingDestinationLine.into()
    );
}

#[test]
fn test_invalid_piece_error() {
    let mut game_state = setup();

    let result = game_state.handle_move("Le5");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidPiece('L').into());
}

#[test]
fn test_missing_capture_character_error() {
    let mut game_state = setup();
    setup_board!(game_state, "e4", "d5", "Nc3", "Nf6");

    let result = game_state.handle_move("Nd5");

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCaptureCharacter.into()
    );
}

#[test]
#[should_panic(expected = "Should call 'initialize' before 'handle_move'")]
fn test_uninitialized_game_state() {
    let mut game_state = GameState::new();
    let _ = game_state.handle_move("Nd5");
}

#[test]
fn test_scholars_mate_full_game() {
    let mut game_state = setup();

    setup_board!(game_state, "e4", "e5", "Bc4", "Nc6", "Qh5", "Nf6", "Qxf7");

    assert!(game_state.verify_checkmate());
}

#[test]
fn test_castling_rights_persistence_after_failed_move() -> Result<(), MoveError> {
    let mut game_state = setup();

    // Clear paths for castling
    setup_board!(game_state, "e4", "e5", "Nf3", "Nf6", "Bc4", "Bc5");

    assert_failed_move_preserves_state(&mut game_state, "Kd2", MoveError::SquareOccupied);

    // Now verify we can STILL castle successfully
    game_state.handle_move("O-O")?;

    Ok(())
}

#[test]
fn test_short_castling_success_on_fixture() -> Result<(), MoveError> {
    let mut game_state = setup_with_positions("tests/validate_castling_path_success.txt");
    let king_source = Position::new(7, 4);
    let rook_source = Position::new(7, 7);

    let king = game_state.get_piece(king_source).unwrap();
    let rook = game_state.get_piece(rook_source).unwrap();

    game_state.handle_move("O-O")?;

    let king_dest = game_state.get_piece(Position::new(7, 6)).unwrap();
    let rook_dest = game_state.get_piece(Position::new(7, 5)).unwrap();

    assert_eq!(
        discriminant(&king.piece_type),
        discriminant(&king_dest.piece_type)
    );

    assert_eq!(king.color, king_dest.color);
    assert_eq!(
        discriminant(&rook.piece_type),
        discriminant(&rook_dest.piece_type)
    );
    assert_eq!(rook.color, rook_dest.color);

    assert!(game_state.get_piece(king_source).is_none());
    assert!(game_state.get_piece(rook_source).is_none());

    assert!(!king_dest.is_short_castling_available());
    assert!(!king_dest.is_long_castling_available());
    assert!(!rook_dest.is_short_castling_available());
    assert!(!rook_dest.is_long_castling_available());

    Ok(())
}

#[test]
fn test_long_castling_success_on_fixture() -> Result<(), MoveError> {
    let mut game_state = setup_with_positions("tests/validate_castling_path_success.txt");
    let king_source = Position::new(7, 4);
    let rook_source = Position::new(7, 0);

    let king = game_state.get_piece(king_source).unwrap();
    let rook = game_state.get_piece(rook_source).unwrap();

    game_state.handle_move("O-O-O")?;

    let king_dest = game_state.get_piece(Position::new(7, 2)).unwrap();
    let rook_dest = game_state.get_piece(Position::new(7, 3)).unwrap();

    assert_eq!(
        discriminant(&king.piece_type),
        discriminant(&king_dest.piece_type)
    );
    assert_eq!(king.color, king_dest.color);
    assert_eq!(
        discriminant(&rook.piece_type),
        discriminant(&rook_dest.piece_type)
    );
    assert_eq!(rook.color, rook_dest.color);
    assert!(game_state.get_piece(king_source).is_none());
    assert!(game_state.get_piece(rook_source).is_none());

    assert!(!king_dest.is_short_castling_available());
    assert!(!king_dest.is_long_castling_available());
    assert!(!rook_dest.is_short_castling_available());
    assert!(!rook_dest.is_long_castling_available());

    Ok(())
}

#[test]
fn test_move_that_exposes_king_is_rejected() {
    let mut game_state = setup_with_positions("tests/pinned_piece_exposes_king.txt");

    assert_failed_move_preserves_state(&mut game_state, "Rf2", MoveError::KingWouldBeInCheck);
}

#[test]
fn test_castling_fails_after_king_has_moved() {
    let mut game_state = setup();
    setup_board!(
        game_state, "e4", "e5", "Nf3", "Nc6", "Bd3", "Nf6", "Ke2", "Be7", "Ke1", "O-O"
    );

    assert_failed_move_preserves_state(
        &mut game_state,
        "O-O",
        MoveError::InvalidCastle("This move is not allowed"),
    );
}

#[test]
fn test_castling_fails_after_rook_has_moved() {
    let mut game_state = setup();
    setup_board!(
        game_state, "Nf3", "Nf6", "g3", "g6", "Bg2", "Bg7", "h3", "h6", "Rh2", "Rh7", "Rh1", "Rh8"
    );

    assert_failed_move_preserves_state(
        &mut game_state,
        "O-O",
        MoveError::InvalidCastle("This move is not allowed"),
    );
}
//
// #[test]
// fn test_trailing_alphabetic_characters_are_rejected() {
//     let mut game_state = setup();
//
//     assert_failed_move_preserves_state(
//         &mut game_state,
//         "e4abc",
//         PgnError::InvalidCharacter('a').into(),
//     );
// }
//
// #[test]
// fn test_trailing_symbol_characters_are_rejected() {
//     let mut game_state = setup();
//
//     assert_failed_move_preserves_state(
//         &mut game_state,
//         "e4?",
//         PgnError::InvalidCharacter('?').into(),
//     );
// }
//
// #[test]
// fn test_trailing_characters_after_long_castle_are_rejected() {
//     let mut game_state = setup_with_positions("tests/validate_castling_path_success.txt");
//
//     assert_failed_move_preserves_state(
//         &mut game_state,
//         "O-O-Oxyz",
//         PgnError::InvalidCharacter('x').into(),
//     );
// }
