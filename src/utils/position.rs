pub struct Position {
    line: usize,
    col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    pub fn line(&self) -> &usize {
        &self.line
    }

    pub fn col(&self) -> &usize {
        &self.col
    }
}

struct ChessPosition {
    line: char,
    col: char,
}

impl ChessPosition {
    fn new(line: char, col: char) -> Self {
        Self { line, col }
    }
}

impl TryFrom<ChessPosition> for Position {
    type Error = String;

    fn try_from(chess_pos: ChessPosition) -> Result<Position, Self::Error> {
        unimplemented!();
    }
}
