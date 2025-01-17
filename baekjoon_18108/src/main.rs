use std::io::{stdin, stdout, Write};

const OFFSET: i32 = 2541 - 1998;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap() - OFFSET;
    stdout().write(input.to_string().as_bytes()).unwrap();
}
