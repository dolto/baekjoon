use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<u32>().unwrap();

    let answer = (2_i32.pow(input as u32) + 1).pow(2);
    stdout().write(answer.to_string().as_bytes()).unwrap();
    stdout().flush().unwrap();
}
