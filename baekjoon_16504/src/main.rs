use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|s| s.parse::<i64>());
    // let mut input = input.split("\n");

    let mut res = 0;
    while let Some(i) = input.next() {
        res += i;
    }

    stdout().write_all(format!("{res}").as_bytes()).unwrap();
}
