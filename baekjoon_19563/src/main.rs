use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i64>());

    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();
    let movefit = a.abs() + b.abs();

    if c >= movefit && (c - movefit) % 2 == 0 {
        stdout().write("YES".as_bytes()).unwrap();
    } else {
        stdout().write("NO".as_bytes()).unwrap();
    }
}
