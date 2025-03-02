use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
};
const A: u8 = b'A' - 10;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<u64>());

    let mut total = input.next().unwrap();
    let base = input.next().unwrap();
    let mut answer = VecDeque::with_capacity(100);

    while total != 0 {
        let temp = (total % base) as u8;
        let temp = if temp < 10 { temp + b'0' } else { temp + A };
        total /= base;
        answer.push_front(temp);
    }

    stdout()
        .write(&answer.into_iter().collect::<Vec<u8>>())
        .unwrap();
}
