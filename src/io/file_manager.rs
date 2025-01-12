use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const POSITIONS_DIRECTORY: &str = "res/positions/";
const DEFAULT_POSITIONS_FILE: &str = "initial_positions.txt";

pub fn initial_positions(positions_file: Option<&str>) -> Lines<BufReader<File>> {
    let positions_file = positions_file.unwrap_or(DEFAULT_POSITIONS_FILE);
    let positions_file_path = format!("{}/{}", POSITIONS_DIRECTORY, positions_file);

    let file = File::open(&positions_file_path)
        .unwrap_or_else(|_| panic!("Could not open file {}", &positions_file_path));

    BufReader::new(file).lines()
}
