use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let (x, y) = (input.next().unwrap(), input.next().unwrap());

    if x > 0 {
        if y > 0 {
            stdout().write("1".as_bytes()).unwrap();
        } else {
            stdout().write("4".as_bytes()).unwrap();
        }
    } else {
        if y > 0 {
            stdout().write("2".as_bytes()).unwrap();
        } else {
            stdout().write("3".as_bytes()).unwrap();
        }
    }
}
