use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    stdout().write((a * b).to_string().as_bytes()).unwrap();
}
