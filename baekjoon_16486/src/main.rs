use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<f64>());

    let res = (input.next().unwrap() * 2.) + (input.next().unwrap() * 2. * 3.141592);
    stdout().write_all(format!("{res}").as_bytes()).unwrap();
}
