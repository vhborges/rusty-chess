use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
const INITIAL_POSITIONS_FILE: &str = "res/initial_positions.txt";

pub fn initial_positions() -> Lines<BufReader<File>> {
    let file = File::open(INITIAL_POSITIONS_FILE)
        .unwrap_or_else(|_| panic!("Could not open file {}", INITIAL_POSITIONS_FILE));

    BufReader::new(file).lines()
}
