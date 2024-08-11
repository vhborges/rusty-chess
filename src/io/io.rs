use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::Chars;

const INITIAL_POSITIONS: &str = "src/io/initial_positions.txt";

pub fn initial_positions() -> Lines<BufReader<File>> {
    let file = File::open(INITIAL_POSITIONS)
        .unwrap_or_else(|_| panic!("Could not open file {}", INITIAL_POSITIONS));

    BufReader::new(file).lines()
}

pub fn get_next_char(line: &String, chars: &mut Chars) -> char {
    chars
        .next()
        .unwrap_or_else(|| panic!("Line {} is incomplete", line))
}
