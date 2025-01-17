use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let (a, b, c) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );
    stdout()
        .write(
            format!(
                "{}\n{}\n{}\n{}",
                (a + b) % c,
                ((a % c) + (b + c)) % c,
                (a * b) % c,
                ((a % c) * (b % c)) % c,
            )
            .as_bytes(),
        )
        .unwrap();
}
