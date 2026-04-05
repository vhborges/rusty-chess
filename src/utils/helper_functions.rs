use std::str::Chars;

pub fn get_next_char(line: &String, chars: &mut Chars) -> char {
    chars
        .next()
        .unwrap_or_else(|| panic!("Line {line} is incomplete"))
}
