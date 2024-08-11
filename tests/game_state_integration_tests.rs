use chess::errors::{ChessPositionError, MoveError, PgnError};
use chess::utils::Position;
use chess::GameState;

macro_rules! setup_board {
        ( $game_state:expr, $( $x:expr ),* ) => {
            {
                $(
                    $game_state.move_piece($x)
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

    game_state.move_piece(str_move)?;

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
fn test_move_piece() -> Result<(), MoveError> {
    // TODO create a function that will run the next two lines before all
    let mut game_state = GameState::new();
    game_state.initialize();

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
fn test_invalid_move() {
    let mut game_state = GameState::new();
    game_state.initialize();

    // TODO segregate below tests into multiple functions

    let mut result = game_state.move_piece("Kd5");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::NoPieceAvailable);

    setup_board!(game_state, "e4", "c5");
    result = game_state.move_piece("exc5");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::NoPieceAvailable);

    setup_board!(game_state, "d4", "cxd4", "Nf3", "e5");
    result = game_state.move_piece("Nd2");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::MoreThanOnePieceAvailable);

    setup_board!(game_state, "Nbd2", "Bd6", "Nxd4", "Nc6");
    result = game_state.move_piece("Ndb3");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::MoreThanOnePieceAvailable);
}

#[test]
fn test_square_occupied_error() {
    let mut game_state = GameState::new();
    game_state.initialize();

    let result = game_state.move_piece("Ke2");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoveError::SquareOccupied);
}

#[test]
fn test_invalid_capture() {
    let mut game_state = GameState::new();
    game_state.initialize();

    // TODO segregate below tests into multiple functions

    let result = game_state.move_piece("exd3");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        MoveError::InvalidCapture("Destination square is empty")
    );

    let result = game_state.move_piece("Kxe2");
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

    let mut result = game_state.move_piece("e");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCharacter("second").into()
    );

    result = game_state.move_piece("eK");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('K').into());

    result = game_state.move_piece("Kx5");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ChessPositionError::MissingDestinationColumn.into()
    );

    result = game_state.move_piece("KxI");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('I').into());

    result = game_state.move_piece("Kxc");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCharacter("fourth").into()
    );

    result = game_state.move_piece("Kxx7");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ChessPositionError::MissingDestinationColumn.into()
    );

    result = game_state.move_piece("KxdL");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidCharacter('L').into());

    result = game_state.move_piece("KdxcM");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ChessPositionError::MissingDestinationLine.into()
    );

    result = game_state.move_piece("Le5");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PgnError::InvalidPiece('L').into());

    setup_board!(game_state, "e4", "d5", "Nc3", "Nf6");
    result = game_state.move_piece("Nd5");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        PgnError::MissingCaptureCharacter.into()
    );
}
