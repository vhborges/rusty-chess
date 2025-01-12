use chess::GameState;
use chess::errors::{ChessPositionError, MoveError, PgnError};
use chess::utils::Position;
use chess::utils::test_helper::setup;

macro_rules! setup_board {
        ( $game_state:expr, $( $x:expr ),* ) => {
            {
                $(
                    $game_state.handle_move($x)
                        .expect(format!("Something's wrong, {} is not a invalid move!", $x).as_str());
                )*
            }
        };
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
        origin_piece.unwrap().piece_type,
        dest_piece.unwrap().piece_type
    );
    assert_eq!(origin_piece.unwrap().color, dest_piece.unwrap().color);

    Ok(())
}

#[test]
fn test_successful_game() -> Result<(), MoveError> {
    let mut game_state = setup(None);

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
    let mut game_state = setup(None);

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
    let mut game_state = setup(None);

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
    let mut game_state = setup(None);

    let result = game_state.handle_move("Ke2");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::SquareOccupied);
}

#[test]
fn test_destination_square_empty_error() {
    let mut game_state = setup(None);

    let result = game_state.handle_move("exd3");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        MoveError::InvalidCapture("Destination square is empty")
    );
}

#[test]
fn test_same_color_piece_capture_error() {
    let mut game_state = setup(None);

    let result = game_state.handle_move("Kxe2");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        MoveError::InvalidCapture("Cannot capture a piece of the same color")
    );
}

#[test]
fn test_missing_second_character_error() {
    let mut game_state = setup(None);

    let result = game_state.handle_move("e");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCharacter("second").into()
    );
}

#[test]
fn test_invalid_character_error() {
    let mut game_state = setup(None);

    let result = game_state.handle_move("eK");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('K').into());

    let result = game_state.handle_move("KxI");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('I').into());

    let result = game_state.handle_move("KxdL");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('L').into());
}

#[test]
fn test_missing_destination_column_error() {
    let mut game_state = setup(None);

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
    let mut game_state = setup(None);

    let result = game_state.handle_move("Kxc");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCharacter("fourth").into()
    );
}

#[test]
fn test_missing_destination_line_error() {
    let mut game_state = setup(None);

    let result = game_state.handle_move("KdxcM");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ChessPositionError::MissingDestinationLine.into()
    );
}

#[test]
fn test_invalid_piece_error() {
    let mut game_state = setup(None);

    let result = game_state.handle_move("Le5");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidPiece('L').into());
}

#[test]
fn test_missing_capture_character_error() {
    let mut game_state = setup(None);

    setup_board!(game_state, "e4", "d5", "Nc3", "Nf6");
    let result = game_state.handle_move("Nd5");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCaptureCharacter.into()
    );
}
