use std::io::{Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();

    let input = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap() + 8);

    let sum = input.sum::<i32>() - 8;

    write!(stdout(), "{} {}", sum / 24, sum % 24).unwrap();
    stdout().flush().unwrap();
}
