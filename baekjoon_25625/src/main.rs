use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let x = input.next().unwrap();
    let y = input.next().unwrap();

    // 1 <= y < 2x 가 성립하므로

    let answer;
    if x > y {
        answer = x + y;
    } else {
        answer = y % x;
    }

    stdout().write(answer.to_string().as_bytes()).unwrap();
}
