use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::with_capacity(105);
    stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .map(|c| (c as u8 - '0' as u8) as i32);

    let mut total = 0;
    for i in input {
        total += i;
    }

    stdout().write(total.to_string().as_bytes()).unwrap();
}
