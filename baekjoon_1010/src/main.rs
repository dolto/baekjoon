use std::io::{stdin, stdout, Read, Write};

fn factorial(n: i128, res: i128, limit: i128) -> i128 {
    if n <= limit {
        res
    } else {
        factorial(n - 1, n * res, limit)
    }
}
fn main() {
    let mut input = String::new();
    let mut output = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let count = input.next().unwrap().trim().parse().unwrap();

    for _ in 0..count {
        let r: i128 = input.next().unwrap().trim().parse().unwrap();
        let n: i128 = input.next().unwrap().trim().parse().unwrap();

        let result = factorial(n, 1, n - r) / factorial(r, 1, 1);
        output.push_str(format!("{result}\n").as_str());
    }

    stdout().write_all(output.as_bytes()).unwrap();
}
