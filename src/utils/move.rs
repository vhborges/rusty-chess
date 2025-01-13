use super::Position;

#[derive(Copy, Clone)]
/// Represents a single piece move
pub struct PieceMove {
    pub source: Position,
    pub destination: Position,
}

#[derive(Copy, Clone)]
/// Represents a complete move, potentially including an additional move (e.g., for castling)
pub struct Move {
    pub primary: PieceMove,
    pub additional: Option<PieceMove>,
}

impl Move {
    pub fn new(source: Position, destination: Position) -> Self {
        Self {
            primary: PieceMove {
                source,
                destination,
            },
            additional: None,
        }
    }

    pub fn new_with_castling(
        source: Position,
        destination: Position,
        additional_source: Position,
        additional_destination: Position,
    ) -> Self {
        Self {
            primary: PieceMove {
                source,
                destination,
            },
            additional: Some(PieceMove {
                source: additional_source,
                destination: additional_destination,
            }),
        }
    }

    pub fn source(&self) -> Position {
        self.primary.source
    }

    pub fn destination(&self) -> Position {
        self.primary.destination
    }

    pub fn is_castling(&self) -> bool {
        self.additional.is_some()
    }
}
