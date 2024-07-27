use std::io::{stdin, stdout, BufReader, Read, Write};

fn factorial(n: i64, result: i64) -> i64 {
    if n <= 1 {
        result
    } else {
        factorial(n - 1, result * n)
    }
}
fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let res = input.next().unwrap().trim().parse().unwrap();
    let result = factorial(res, 1);

    stdout().write_all(format!("{result}").as_bytes()).unwrap();
}
