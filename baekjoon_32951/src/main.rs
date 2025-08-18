use std::io::{Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let year = input.trim().parse::<i32>().unwrap();

    write!(stdout(), "{}", year - 2024).unwrap();
}
