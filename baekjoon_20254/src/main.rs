use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i64>());

    let result = 56 * input.next().unwrap()
        + 24 * input.next().unwrap()
        + 14 * input.next().unwrap()
        + 6 * input.next().unwrap();

    stdout()
        .write_all(format!("{}", result).as_bytes())
        .unwrap();
}
