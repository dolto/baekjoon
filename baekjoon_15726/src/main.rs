use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<f64>());

    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();

    let res1 = a * b / c;
    let res2 = a / b * c;

    stdout()
        .write_all(format!("{}", res1.max(res2) as i32).as_bytes())
        .unwrap();
}
