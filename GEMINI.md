# GEMINI.md - Rusty Chess

## Project Overview

**Rusty Chess** is a text-based Chess game implemented in Rust. It serves as a learning project for Rust concepts while aiming to become a full-featured chess application with a GUI, AI engine, and multiplayer support.

### Architecture
- **`GameState` (`src/game_state.rs`):** The central hub for game logic. It manages the board, turn rotation, captured pieces, and identifies game-ending conditions like checkmate and stalemate.
- **`Board` (`src/board.rs`):** Represents the 8x8 chess board and provides methods for piece placement and path validation.
- **`Piece` System (`src/pieces/`):** Each chess piece (King, Queen, Rook, Bishop, Knight, Pawn) has specialized movement and attack logic defined in `src/pieces/types/`.
- **`PGN Parser` (`src/pgn/`):** A custom multi-step parser that converts standard PGN (Portable Game Notation) move strings into executable `Move` objects.
- **`IO` (`src/io/`):** Handles terminal-based rendering (`ui.rs`) and file-based initialization (`file_manager.rs`).

### Tech Stack
- **Language:** Rust (Edition 2024).
- **Toolchain:** Requires **Rust 1.85+ nightly** (configured in `rust-toolchain.toml`).
- **Dependencies:** `clearscreen` for terminal management.

---

## Building and Running

### Prerequisites
Ensure you have the nightly Rust toolchain installed:
```bash
rustup toolchain install nightly
```

### Key Commands
- **Run the game:** `cargo run`
- **Build the project:** `cargo build`
- **Run all tests:** `cargo test`
- **Format code:** `cargo fmt`

---

## Development Conventions

### Coding Style
- **Rust Idioms:** Strictly follows standard Rust conventions and the 2024 edition features.
- **Formatting:** Managed by `rustfmt.toml`. Always run `cargo fmt` before committing.
- **Error Handling:** Custom error types are defined in `src/errors/` (e.g., `MoveError`, `PGNError`).

### Testing Practices
- **Unit Tests:** Located within individual source files (e.g., at the bottom of `game_state.rs`).
- **Integration Tests:** Located in `tests/integration_tests.rs`.
- **Test Helpers:** `src/utils/test_helper.rs` provides utilities to load specific board configurations from files for testing.
- **Position Files:** Test cases often use setup files located in `res/positions/tests/` to validate specific scenarios (stalemate, castling, etc.).

### Configuration
- **Initial Positions:** The starting board layout is defined in `res/positions/initial_positions.txt`. The format is `[Color][PieceType][Column][Rank]` (e.g., `WPa2` for a White Pawn on a2).

---

## Roadmap & Progress
- [x] Textual UI & Basic Movement
- [x] PGN Parsing (Captures, Disambiguation, Castling)
- [x] Checkmate & Stalemate detection
- [ ] En Passant support (Planned)
- [ ] Graphical Interface (Planned)
- [ ] AI Engine (Planned)
