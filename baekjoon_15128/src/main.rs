use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();

    let mut input = input.split_ascii_whitespace();

    let p1: f64 = input.next().unwrap().trim().parse().unwrap();
    let q1: f64 = input.next().unwrap().trim().parse().unwrap();
    let p2: f64 = input.next().unwrap().trim().parse().unwrap();
    let q2: f64 = input.next().unwrap().trim().parse().unwrap();

    let result = p1 * p2 / q1 / q2 / 2.;
    if result == result.trunc() {
        stdout().write_all("1".as_bytes()).unwrap();
    } else {
        stdout().write_all("0".as_bytes()).unwrap();
    }
}
