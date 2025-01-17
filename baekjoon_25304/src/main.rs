use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let mut total = input.next().unwrap();
    let n = input.next().unwrap();
    (0..n).for_each(|_| {
        total -= input.next().unwrap() * input.next().unwrap();
    });

    if total == 0 {
        stdout().write("Yes".as_bytes()).unwrap();
    } else {
        stdout().write("No".as_bytes()).unwrap();
    }

    stdout().flush().unwrap();
}
