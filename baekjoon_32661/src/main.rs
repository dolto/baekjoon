use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>())
        .skip(1);

    let mut max = i32::MIN;
    let mut min = i32::MAX;
    for s in input {
        max = max.max(s);
        min = min.min(s);
    }

    max /= 2;

    stdout()
        .write((min - max).max(0).to_string().as_bytes())
        .unwrap();
}
