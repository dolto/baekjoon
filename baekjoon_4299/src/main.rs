use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let a = input.next().unwrap();
    let b = input.next().unwrap();

    if (a + b) % 2 == 0 {
        let x = (a + b) / 2;
        let y = x - b;
        if x < 0 || y < 0 {
            stdout().write_all("-1".as_bytes()).unwrap();
        } else {
            stdout().write_all(format!("{x} {y}").as_bytes()).unwrap();
        }
    } else {
        stdout().write_all("-1".as_bytes()).unwrap();
    }
}
