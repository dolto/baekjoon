use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let a = input.next().unwrap();
    let b = input.next().unwrap();
    stdout().write(format!("{}", a + b).as_bytes()).unwrap();
}
