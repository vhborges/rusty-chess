pub enum Color {
    White,
    Black,
}

impl Color {
    fn flip(&self) -> Self {
        match *self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
