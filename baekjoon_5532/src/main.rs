use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|c| c.parse::<i32>());

    let mut left_day = input.next().unwrap();
    let mut speech = input.next().unwrap();
    let mut math = input.next().unwrap();

    let max_speech = input.next().unwrap();
    let max_math = input.next().unwrap();

    while speech > 0 || math > 0 {
        left_day -= 1;
        speech -= max_speech;
        math -= max_math;
    }

    stdout()
        .write_all(format!("{}", left_day.max(0)).as_bytes())
        .unwrap();
}
