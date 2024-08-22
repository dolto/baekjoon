use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i64>())
        .skip(1);
    let mut total = 0;
    while let Some(temp) = input.next() {
        total += temp;
    }

    stdout().write_all(format!("{}", total).as_bytes()).unwrap();
}
