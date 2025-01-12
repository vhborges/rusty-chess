use std::io::{self, Write, stdin, stdout};

pub fn read_move() -> io::Result<String> {
    print!("Next move: ");
    stdout().flush()?;

    let mut next_move = String::new();
    stdin().read_line(&mut next_move)?;

    Ok(next_move.trim().to_owned())
}
