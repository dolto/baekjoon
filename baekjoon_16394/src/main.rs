use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i64>().unwrap();

    stdout()
        .write_all(format!("{}", input - 1946).as_bytes())
        .unwrap();
}
