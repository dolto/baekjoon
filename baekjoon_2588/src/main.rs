use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    let (c, d, e, f) = (a * (b % 10), a * (b % 100 / 10), a * (b / 100), a * b);

    stdout()
        .write(format!("{}\n{}\n{}\n{}", c, d, e, f).as_bytes())
        .unwrap();
}
