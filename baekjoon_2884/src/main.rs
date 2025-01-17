use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let (mut h, mut m) = (input.next().unwrap(), input.next().unwrap());

    m -= 45;
    if m < 0 {
        h -= 1;
        m += 60;
    }

    if h < 0 {
        h = 23;
    }

    stdout().write(format!("{h} {m}").as_bytes()).unwrap();
}
