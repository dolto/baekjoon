use std::io::{stdin, stdout, Read, Write};

fn factorial(n: u64, res: u64) -> u64 {
    if n <= 1 {
        res
    } else {
        factorial(n - 1, n * res)
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    stdout()
        .write_all(format!("{}", factorial(n, 1)).as_bytes())
        .unwrap();
}
