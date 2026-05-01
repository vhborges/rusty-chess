# Repository Guidelines

## Project Structure & Module Organization
Core game logic lives in `src/`. Main domains are split by concern: `pieces/` (piece rules), `movement/` (positions and directions), `pgn/` (move parsing), `io/` (terminal input/output), and `errors/` (typed failures). Entry points are `src/main.rs` (CLI app) and `src/lib.rs` (library surface). Integration tests live in `tests/integration_tests.rs`. Board setup and scenario fixtures are in `res/positions/`, including `res/positions/initial_positions.txt` and `res/positions/tests/`.

## Build, Test, and Development Commands
- `cargo run`: start the text-based chess game locally.
- `cargo test`: run unit and integration tests.
- `cargo test --test integration_tests`: run only end-to-end move flow tests.
- `cargo fmt`: format code using repository `rustfmt` settings.
- `cargo clippy --all-targets --all-features`: run extra lint checks before opening a PR.

Use project root as working directory for all commands.

## Coding Style & Naming Conventions
Follow idiomatic Rust and keep modules focused on one responsibility. Use `snake_case` for modules, files, and functions, and `CamelCase` for structs/enums/traits. Prefer explicit error types over stringly-typed failures. Format with `cargo fmt`; this repo uses `rustfmt.toml` with `control_brace_style = "ClosingNextLine"`, so do not hand-format around brace style.

## Testing Guidelines
Write fast unit tests close to implementation using `#[cfg(test)]` blocks in module files, and keep integration behavior tests in `tests/`. Name tests by behavior, e.g. `test_successful_game` or `test_square_occupied_error`. For move-related changes, cover both valid PGN flows and invalid-move rollback behavior (state should remain consistent after failed moves).

## Commit & Pull Request Guidelines
Use short, imperative commit subjects (e.g. `Implement stalemate`, `Refactor pieces module`) and keep each commit focused on one change. PRs should include: a concise summary, affected areas (for example `pgn` or `game_state`), and the commands you ran (`cargo test`, plus any targeted tests). Link an issue when relevant, but it is not mandatory in this repository.
