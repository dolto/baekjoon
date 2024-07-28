use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let left: f64 = input.next().unwrap().parse().unwrap();
    let right: f64 = input.next().unwrap().parse().unwrap();

    stdout()
        .write_all(format!("{}", left / right).as_bytes())
        .unwrap();
}
