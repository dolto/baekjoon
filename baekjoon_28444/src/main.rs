use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let h = input.next().unwrap();
    let i = input.next().unwrap();
    let a = input.next().unwrap();
    let r = input.next().unwrap();
    let c = input.next().unwrap();

    let left = h * i;
    let right = a * r * c;
    let res = left - right;

    stdout().write_all(format!("{res}").as_bytes()).unwrap();
}
