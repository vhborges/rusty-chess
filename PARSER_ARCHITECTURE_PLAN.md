# Parser Architecture Refactor Plan

## Goal
Refactor the PGN parser into clear layers so syntax parsing and board-dependent move resolution are separated, easier to test, and easier to extend (promotion/en passant).

## Target Architecture

### 1. Lexer Layer (`&str -> Vec<Token>`)
Purpose: classify raw input characters/tokens, without chess legality checks.

Proposed token set:

```rust
enum Token {
    Piece(char),      // K Q R B N (pawn is implicit)
    File(char),       // a..h
    Rank(char),       // 1..8
    Capture,          // x
    CastleShort,      // O-O
    CastleLong,       // O-O-O
    Check,            // +
    Mate,             // #
    End,
}
```

Examples:
- `"Nf3"` -> `[Piece('N'), File('f'), Rank('3'), End]`
- `"exd5"` -> `[File('e'), Capture, File('d'), Rank('5'), End]`
- `"O-O+"` -> `[CastleShort, Check, End]`
- `"e4abc"` -> lexer error at `'a'` (or invalid token rejected by parser)

### 2. Syntax Parser Layer (`Vec<Token> -> MoveSpec`)
Purpose: validate PGN grammar only; no `GameState` access.

Proposed structures:

```rust
enum CastleSide { Short, Long }
enum Suffix { Check, Mate }

struct MoveSpec {
    piece: PieceKind,
    to_file: Option<char>,
    to_rank: Option<char>,
    from_file_hint: Option<char>, // disambiguation
    from_rank_hint: Option<char>, // disambiguation
    capture: bool,
    castle: Option<CastleSide>,
    suffix: Option<Suffix>,
}
```

Examples:
- `"Nbd2"` -> Knight to `d2`, `from_file_hint=b`
- `"R1e1"` -> Rook to `e1`, `from_rank_hint=1`
- `"O-O-O"` -> `castle=Long`
- `"Kx"` -> syntax error (missing destination)
- `"e4?"` -> syntax error (unexpected trailing token)

Requirement: parser must consume all tokens until `End`; leftovers are invalid.

### 3. Resolver/Validator Layer (`MoveSpec + GameState -> Move`)
Purpose: convert parsed intent into executable move using board state.

Responsibilities:
- Find candidate source pieces in `GameState`.
- Apply disambiguation hints.
- Enforce movement/capture rules.
- Validate castling rights and castling path.
- Reject self-check (`MoveError::KingWouldBeInCheck`).
- Return internal `Move` used by board execution.

Examples:
- Ambiguous knight destination -> `MoreThanOnePieceAvailable`
- Capture requested but destination empty -> `InvalidCapture`
- Castling after king/rook moved -> `InvalidCastle`

## Proposed Module Layout
- `src/pgn/lexer.rs`
- `src/pgn/parser.rs` (syntax parser)
- `src/pgn/spec.rs` (`MoveSpec`, parser enums)
- `src/pgn/resolver.rs` (board-aware resolution)
- `src/pgn/mod.rs` exposing `parse_move(game_state, input)`

## Error Model
Introduce phase-specific errors:
- `PgnLexError`
- `PgnSyntaxError`
- `MoveError` (board legality and runtime move validation)

Wrap lex/syntax failures as `MoveError::InvalidPgn(PgnError)` while preserving existing move legality variants.

## Test Strategy
- Lexer tests: tokenization + invalid characters.
- Syntax parser tests: disambiguation forms, missing fields, castling, suffix handling, trailing-token rejection.
- Resolver tests: ambiguity, capture rules, castling rights/path, self-check prevention.
- Integration tests: full `handle_move("...")` end-to-end behavior.
