use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i64>())
        .skip(1);

    let moduler = input.next().unwrap();
    let mut result = 1;
    while let Some(temp) = input.next() {
        result *= temp % moduler;
        result %= moduler;
    }

    stdout().write(format!("{result}").as_bytes()).unwrap();
}
