use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: i64 = input.next().unwrap().trim().parse().unwrap();
    let m: i64 = input.next().unwrap().trim().parse().unwrap();

    let tiles = n * m / 2;

    stdout().write_all(format!("{tiles}").as_bytes()).unwrap();
}
