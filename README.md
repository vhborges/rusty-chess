# Rusty Chess
A (so far) text-based Chess game written in Rust.

This project was created with the purpose of learning and practicing Rust concepts as I code, but my long-term goal is to have a fully working game, with a graphical interface, mouse support, AI engine and multiplayer functionality.

## Roadmap
- [x] Textual interface
- [ ] PGN movement and validations
  - [x] Basic movement
  - [x] Capture symbol support
  - [x] Disambiguation symbol support (e.g. N**f**e5)
  - [x] Invalid move validation
  - [x] Castling support
  - [ ] Check symbol support
  - [ ] Checkmate support
  - [ ] Stalemate support
- [x] Tests
  - [x] Integration tests
  - [ ] Unit tests (in progress)
- [ ] Graphical interface & mouse support
- [ ] AI engine
- [ ] Multiplayer

## Requirements
- Rust 1.85+ (**nightly** toolchain)

## Running & playing
Simply run: `cargo run`

You will be presented with the following screen:

![image](https://github.com/user-attachments/assets/dca97cd6-d93e-44a7-b853-6bf3945f95dc)

Turns alternate between white and black, in that order. In each turn, the game expects you to enter a PGN move, for example:
- e4 (move the King's pawn to *e4*)
- Nf3 (move the Knight to *f3*)
- Bxc4 (bishop captures the piece on *c4*)
- N6e5 (move the Knight currently on the sixth row to *e5* - assuming both Knights can go to *e5*)
- Nfe5 (move the *f*-column Knight to *e5* - assuming both Knights can go to *e5*)
