use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let result = (input.next().unwrap() * 8) + (input.next().unwrap() * 3) - 28;
    stdout().write_all(format!("{result}").as_bytes()).unwrap();
}
