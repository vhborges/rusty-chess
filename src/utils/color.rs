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

impl TryFrom<char> for Color {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'W' => Ok(Color::White),
            'B' => Ok(Color::Black),
            _ => Err(format!("Invalid color character: {}", value)),
        }
    }
}
