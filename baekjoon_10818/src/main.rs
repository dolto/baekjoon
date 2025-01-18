use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>())
        .skip(1);

    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for i in input {
        min = min.min(i);
        max = max.max(i);
    }

    write!(stdout(), "{} {}", min, max).unwrap();
    stdout().flush().unwrap();
}
