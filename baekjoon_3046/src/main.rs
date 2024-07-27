use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();

    let mut input = input.split_ascii_whitespace();

    let r1: i32 = input.next().unwrap().trim().parse().unwrap();
    let s: i32 = input.next().unwrap().trim().parse().unwrap();

    let r2 = s * 2 - r1;

    stdout().write_all(format!("{r2}").as_bytes()).unwrap();
}
