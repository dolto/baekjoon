use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::with_capacity(10);
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let (k, q, l, b, n, p) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );

    write!(
        stdout(),
        "{} {} {} {} {} {}",
        1 - k,
        1 - q,
        2 - l,
        2 - b,
        2 - n,
        8 - p
    )
    .unwrap();
    stdout().flush().unwrap();
}
