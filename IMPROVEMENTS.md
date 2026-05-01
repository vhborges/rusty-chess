# Rusty Chess: Detailed Improvement & Refactoring Guide

This document provides a comprehensive guide to improving the Rusty Chess codebase. Each suggestion includes a description of the current approach, proposed solutions, and a detailed explanation of the benefits.

---

## 1. Directional Offsets (Unifying Sliding Pieces)

### Current Code
In `src/pieces/types/bishop.rs` (and similarly for Rook), moves are generated using nested loops and manual scanning:
```rust
for dx in (-1..=1).step_by(2) {
    for dy in (-1..=1).step_by(2) {
        let direction = Direction::new(dx, dy, pos_i8);
        for pos in direction {
            if board.is_position_occupied(pos) { break; }
            result.push(pos);
        }
    }
}
```

### Proposed Solution
Define a shared "sliding move" function that takes a list of coordinate offsets.

```rust
type Offset = (i8, i8);
const BISHOP_DIRS: [Offset; 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const ROOK_DIRS: [Offset; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn get_sliding_moves(board: &Board, origin: Position, directions: &[Offset]) -> Vec<Position> {
    let mut moves = Vec::new();
    for &(dc, dl) in directions {
        let mut current = origin;
        while let Some(next) = current.apply_offset(dc, dl) {
            if let Some(piece) = board.get_piece(next) {
                if piece.color != self.color { moves.push(next); } // Capture
                break; // Path blocked
            }
            moves.push(next);
            current = next;
        }
    }
    moves
}
```

### Contextualization
**What problem does it solve?** It eliminates code duplication. Currently, Rooks and Bishops have almost identical logic.
**Why is it better?** It's **DRY (Don't Repeat Yourself)**. If you need to change how path blocking works (e.g., adding X-ray attacks later), you only change it in one place. It also makes the `Queen` implementation trivial: just pass both arrays of offsets.

---

## 2. Error Propagation vs. Panics

### Current Code
The codebase uses `expect()` with internal error codes in several critical logic paths:
```rust
// src/game_state.rs
let king_pos = self.get_king_pos(temporary_board, next_move.destination());
if self.is_king_in_check(&temporary_board, king_pos, self.turn) { ... }

// Inside is_king_in_check:
.expect(INTERNAL_ERROR_02)
```

### Proposed Solution
Update logic functions to return `Result<bool, MoveError>`.

```rust
fn is_king_in_check(&self, board: &Board, king_pos: Position, color: Color) -> Result<bool, MoveError> {
    for (piece, pos) in board.into_iter().filter(|(p, _)| p.color != color) {
        if piece.attacks(board, pos, king_pos, false, false)? { // Use '?' operator
            return Ok(true);
        }
    }
    Ok(false)
}
```

### Contextualization
**What problem does it solve?** Unexpected application crashes.
**Why is it better?** In Rust, `expect` is for "this should never happen." However, chess logic is complex. If a bug exists in piece movement, a `Result` allows the game to show an error message like "Internal Error" to the player, while a panic crashes the entire terminal session.

---

## 3. Pattern Matching with `matches!`

### Current Code
Type checking often uses `discriminant()`, which is lower-level and harder to read:
```rust
if discriminant(&piece.piece_type) != discriminant(&piece_type) { ... }
```

### Proposed Solution
Use the `matches!` macro or derive `PartialEq` for your enums.

```rust
// Much more readable:
if piece.piece_type != piece_type { ... }

// Or for specific types:
if matches!(piece.piece_type, PieceType::Pawn(_)) { ... }
```

### Contextualization
**What problem does it solve?** Code verbosity and readability.
**Why is it better?** Pattern matching is a core strength of Rust. Using `discriminant` bypasses the expressive power of the type system and makes the code feel more like C than idiomatic Rust.

---

## 4. Robust Test Paths

### Current Code
Tests rely on relative paths to the current working directory:
```rust
let game_state = setup_game_state(Some("tests/validate_stalemate.txt"));
```

### Proposed Solution
Use `env!("CARGO_MANIFEST_DIR")` to build absolute paths relative to the project root.

```rust
#[test]
fn test_stalemate() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/res/positions/tests/validate_stalemate.txt");
    let game_state = setup_game_state(Some(path));
}
```

### Contextualization
**What problem does it solve?** Tests failing when run from different subdirectories or in CI/CD environments.
**Why is it better?** It ensures your test suite is "portable." No matter where `cargo test` is invoked, the files will always be found.

---

## 5. Advanced/Performance Suggestions

### A. Compact Position Representation
**Current:** `Position { line: usize, col: usize }` (16 bytes on 64-bit systems).
**Solution:** Store position as a single `u8` (0-63). Use `/ 8` for line and `% 8` for column.
**Why?** It fits multiple positions into a single CPU register and reduces cache misses.

### B. Zobrist Hashing
**Problem:** Detecting draws by repetition or caching AI results.
**Solution:** Generate a large table of random numbers. XOR them together based on piece positions to get a unique 64-bit ID for any board state.
**Why?** Allows you to implement a "Transposition Table" (a cache) for your AI, often doubling its search depth.

### C. Move Generation via Callbacks
**Problem:** `get_possible_moves` returns a `Vec`, allocating memory on the heap.
**Solution:** Use `fn for_each_move<F>(mut f: F) where F: FnMut(Move)`.
**Why?** It avoids heap allocations entirely, keeping the move generation "zero-cost."
