use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::with_capacity(8);
    stdin().read_line(&mut input).unwrap();

    let mut input = input.split_ascii_whitespace();
    let (a, b) = (
        convert(input.next().unwrap()),
        convert(input.next().unwrap()),
    );

    stdout().write(a.max(b).to_string().as_bytes()).unwrap();
    stdout().flush().unwrap();
}

fn convert(s: &str) -> i32 {
    let s = s.chars().rev().collect::<String>();
    s.parse().unwrap()
}
